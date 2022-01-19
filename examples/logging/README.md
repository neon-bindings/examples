# Logging

The logging example connects the Rust [`log`][log] crate to the Node [`debug`][debug] module. Neon modules can write logs directly to `stdout` with typical crates like [`env_logger`][env-logger], but connecting logging to [`debug`][debug] more seamlessly integrates with the Node ecosystem.

## Usage

```sh
# Only `INFO` logs on the top level crate
DEBUG="INFO:logging" npm start

# All logs on the top level crate
DEBUG="*:logging" npm start

# All `INFO` logs on any Rust crate
DEBUG="INFO:*" npm start

# All `WARN` and higher logs in our crate
DEBUG="WARN:logging,ERROR:logging"
```

## Libraries

### [`log`][log] crate

The [`log`][log] crate provides a logging facade used throughout the Rust ecosystem. It provides convenient macros for logging, but does not provide any facility to write or display logs.

### [`debug`][debug] module

the [`debug`][debug] node module provides a decorated version of `console.error` and is used throughout the Node library ecosystem, including in the [`express`][express] HTTP framework. It allows configurable log filtering with the `DEBUG` environment variable.

## Design

Rust code uses the typical logging facilities, but in order for it to be used, the [`Log`][log-trait] must be implemented. This example provides a simple [`Log`][log-trait] implementation that delegates to the [`debug`][debug] node module.

### Initialization

At initialization, the module calls `global.require("debug")` to get a copy of the function used to create logger instances. This function, as well as `enabled` and a [`Channel`][channel] are used to create a `Logger` instance and initialize the `log` crate.

### Loggers

The `Logger` struct maintains a map of logger names to logger instances that are lazily created as needed. Each logger is in an `Option` with `None` representing the disabled state. If an entry is missing, it is assumed to be in the `enabled` state until it can be further evaluated.

## Limitations

### Levels

The provided implementation does not understand logger levels. Each level needs to be enabled individually. As an improvement, the logger level could be determined by checking `debug.enabled(...)` for each level from lowest to highest.

### Multiple Contexts

The `log` crate only supports a single global logger instance in a process. If the module is initialized multiple times with Web Workers, all logs will be sent to the instance that initialized. 

### Runtime level changes

The `debug` module supports enabling and disabling logging at runtime, but for efficiency, our `Logging` implementation assumes that the result of `debug.enabled(..)` never changes.

[log]: https://crates.io/crates/log
[log-trait]: https://docs.rs/log/latest/log/trait.Log.html
[debug]: https://www.npmjs.com/package/debug
[env-logger]: https://crates.io/crates/env-logger
[express]: https://www.npmjs.com/package/express
[channel]: https://docs.rs/neon/latest/neon/event/struct.Channel.html
