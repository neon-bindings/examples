extern crate neon;

use std::ops::Drop;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use neon::prelude::*;

// Eager Initialization
// Instance directly created from a factory
pub struct Logger {
    name: String,
    // Represents a value created or borrowed from a factory
    // For example, a database connection.
    counter: Arc<AtomicU64>,
}

impl Logger {
    pub fn log(&self, s: &str) {
        let count = self.counter.load(Ordering::Relaxed);

        println!("Loggers: {}, {}: {}", count, self.name, s);
    }
}

// Simulate end-of-life on a factory instance. This would often
// be handled by a `Pool` library (e.g., r2d2).
impl Drop for Logger {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::Relaxed);
    }
}

// Lazy Initialization
// Instance created and then later filled with data
// This pattern can be useful for structs that cannot be immediately
// created (e.g., relies on a side-effect).
pub struct LazyLogger {
    pub logger: Option<Logger>,
}

// Factory for creating instances
pub struct LogFactory {
    // Represents shared state for creating instances.
    // For example, a connection pool.
    counter: Arc<AtomicU64>,
}

impl LogFactory {
    pub fn new() -> Self {
        LogFactory {
            counter: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn create(&self, name: String) -> Logger {
        self.counter.fetch_add(1, Ordering::Relaxed);

        Logger {
            name,
            counter: self.counter.clone(),
        }
    }
}

declare_types! {
    pub class JsLogger for Logger {
        // Constructor for creating a new logger. This should only ever
        // be called from within `Rust` via the `JsLogFactory`
        // new Logger(factory: LogFactory, name: String): Logger
        init(mut cx) {
            let factory = cx.argument::<JsLogFactory>(0)?;
            let name = cx.argument::<JsString>(1)?.value();

            // Grab the inner `LogFactory` from `JsLogFactory`
            let guard = cx.lock();
            let factory = factory.borrow(&guard);

            // Create a new `Logger` from the factory
            Ok(factory.create(name))
        }

        method log(mut cx) {
            let log = cx.argument::<JsString>(0)?;
            let this = cx.this();
            let guard = cx.lock();

            // Borrow the internal `Logger` and call `log`
            this.borrow(&guard).log(&log.value());

            Ok(cx.undefined().upcast())
        }
    }

    pub class JsLazyLogger for LazyLogger {
        // Constructor for creating a new logger. This is initially
        // created empty and needs to be filled later for use.
        init(_) {
            Ok(LazyLogger {
                logger: None,
            })
        }

        method log(mut cx) {
            let log = cx.argument::<JsString>(0)?;
            let this = cx.this();

            // Borrow the internal `logger`
            cx.borrow(&this, |logger| {
                // If the inner `logger` has not been filled, this will panic
                let logger = logger.logger.as_ref().unwrap();

                logger.log(&log.value());
            });

            Ok(cx.undefined().upcast())
        }
    }

    pub class JsLogFactory for LogFactory {
        // Constructs a new factory. This could take arguments
        // (e.g. connection string)
        init(_) {
            Ok(LogFactory::new())
        }

        // Create an instance of a logger
        method create(mut cx) {
            let name = cx.argument::<JsValue>(0)?;
            let this = cx.this().upcast();

            // Call the constructor on `JsLogger`. This should only be called
            // from rust. The first argument, `this`, points to the factory.
            Ok(JsLogger::new(&mut cx, vec![this, name])?.upcast())
        }

        // Create an instance of a logger and then lazily set the inner logger
        method lazy(mut cx) {
            let name = cx.argument::<JsString>(0)?;
            let this = cx.this();
            let inner = cx.borrow(&this, |factory| factory.create(name.value()));

            // Creating the instance requires an extra type hint due to the
            // ambiguity introduced by borrowing the internal value
            let mut logger = JsLazyLogger::new::<_, JsLazyLogger, _>(&mut cx, vec![])?;
            let guard = cx.lock();

            // Borrow the internal logger and set the inner logger
            logger.borrow_mut(&guard).logger = Some(inner);

            Ok(logger.upcast())
        }
    }
}

register_module!(mut cx, {
    // Only expose `LogFactory` since `Logger` should not be directly created.
    // Note: It's technically possible to still access the `Logger` constructor
    // via `logger.constructor` on a logger instance.
    cx.export_class::<JsLogFactory>("LogFactory")?;

    Ok(())
});
