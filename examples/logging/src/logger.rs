use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use log::{LevelFilter, Log, Metadata, Record};
use neon::prelude::*;

type Debug = Option<Root<JsFunction>>;

pub struct Logger {
    // Used for calling back to the main JavaScript thread
    channel: Channel,

    // Reference counted internal state for the logger used to emit logs
    internal: Arc<LoggerInternal>,
}

struct LoggerInternal {
    // Reference to `require("debug")` function
    debug: Root<JsFunction>,

    // Reference to `debug.enabled` function
    enabled: Root<JsFunction>,

    // Set of initialized instances of `debug`
    loggers: RwLock<HashMap<String, Debug>>,
}

impl LoggerInternal {
    // Get an instance of `debug` if it is enabled, lazily creating if necessary
    fn debug<'a, C: Context<'a>>(
        &self,
        cx: &mut C,
        key: String,
    ) -> NeonResult<Option<Handle<'a, JsFunction>>> {
        // If available, return teh cached value
        if let Some(logger) = self.loggers.read().unwrap().get(&key) {
            return Ok(logger.as_ref().map(|root| root.to_inner(cx)));
        };

        // Call `debug.enabled(name)` to see if logging is enabled.
        let name = cx.string(&key);
        let is_enabled = self
            .enabled
            .to_inner(cx)
            .call_with(cx)
            .arg(name)
            .apply::<JsBoolean, _>(cx)?
            .value(cx);

        // If the logger is not enabled, insert `None` and return.
        if !is_enabled {
            self.loggers.write().unwrap().insert(key, None);

            return Ok(None);
        }

        // Create an instance of the logger by calling `debug(name)`
        let debug = self
            .debug
            .to_inner(cx)
            .call_with(cx)
            .arg(name)
            .apply::<JsFunction, _>(cx)?;

        // Insert the logger instance into the cache
        self.loggers
            .write()
            .unwrap()
            .insert(key, Some(debug.root(cx)));

        Ok(Some(debug))
    }
}

impl Logger {
    /// Creates an instance of [`Logger`] and sets it as the global logger in [`log`]
    pub fn init<'a, C: Context<'a>>(cx: &mut C, debug: Handle<JsFunction>) -> NeonResult<()> {
        let logger = Box::new(Logger::new(cx, debug)?);

        log::set_logger(Box::leak(logger))
            .map(|_| log::set_max_level(LevelFilter::Trace))
            .or_else(|err| cx.throw_error(err.to_string()))
    }

    /// Creates an instance of [`Logger`]
    pub fn new<'a, C: Context<'a>>(cx: &mut C, debug: Handle<JsFunction>) -> NeonResult<Self> {
        // Create a new channel. This channel is a queue that is not shared with other
        // calls, allowing more efficient collapsing of calls.
        let mut channel = Channel::new(cx);

        // Prevent logging from preventing the process from being stopped. Some logs
        // may be lost at shutdown.
        channel.unref(cx);

        // Get a reference to `debug.enabled`
        let enabled = debug
            .get(cx, "enabled")?
            .downcast_or_throw::<JsFunction, _>(cx)?;

        let internal = Arc::new(LoggerInternal {
            debug: debug.root(cx),
            enabled: enabled.root(cx),
            loggers: Default::default(),
        });

        Ok(Self { channel, internal })
    }

    // Name `debug` instances like `INFO:logging`
    fn name(metadata: &Metadata) -> String {
        format!("{}:{}", metadata.level(), metadata.target())
    }

    // Check if the logger _might_ be enabled
    fn enabled(&self, name: &str) -> bool {
        // If the key exists in the map and the value is `Some(_)` _or_ if the key
        // does not exist, return `true`.
        self.internal
            .loggers
            .read()
            .unwrap()
            .get(name)
            .map(Option::is_some)
            .unwrap_or(true)
    }
}

// The [`Log`] trait must be implemented by loggers installed globally to the `log` crate.
impl Log for Logger {
    // Check if the logger _might_ be enabled, delegating to a version that takes a `&str`
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.enabled(&Self::name(metadata))
    }

    fn log(&self, record: &Record) {
        let name = Self::name(record.metadata());

        // Logging is fairly expensive. Short-circuit if the log level is not enabled.
        // Uses the `&str` version of `enabled` to avoid allocating an extra `String`
        if !self.enabled(&name) {
            return;
        }

        let internal = self.internal.clone();
        let msg = record.args().to_string();

        // Calling out to `debug` to log must happen on the main JavaScript thread. The
        // closure is moved for execution.
        let _ = self.channel.try_send(move |mut cx| {
            if let Some(debug) = internal.debug(&mut cx, name)? {
                debug.call_with(&cx).arg(cx.string(msg)).exec(&mut cx)?;
            }

            Ok(())
        });
    }

    fn flush(&self) {}
}
