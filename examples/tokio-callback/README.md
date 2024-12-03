# `tokio-callback`

More complex example of spawning a Rust async task on the [tokio][tokio] thread pool and resolving a JavaScript [Promise][promise] after it completes, possibly
returning a callback result to the Rust task. This allows for an asynchronous preparation phase (like retrieving data from a database), a JavaScript processing callback (like an external service invocation, possibly asynchronous), and, when needed, an asynchronous finalization phase (for instance, writing data back to the db).
This example builds on `tokio-fetch` example so refer to that for a simpler scenario.
Uses [neon-serde2][neon-serde2] for serialization and [anyhow][anyhow] for error handling.

[tokio]: https://tokio.rs
[promise]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise
[neon-serde2]: https://github.com/druide/neon-serde
[anyhow]:[https://github.com/dtolnay/anyhow]

## Methods

#### `tryAsJsPromise()`

Allows asynchronous execution of Rust code. This is a streamlined interface that covers the same exact scenario of `tokio-fetch` example.

```javascript
let response = await module.tryAsJsPromise(request);
```

#### `tryAsJsCallback()`

Allows asynchronous execution of Rust code in preparation of a synchronous javascript callback. The result of the Rust preparation is passed as argument of the JavaScript lambda function that represents the callback

```javascript
let response = await module.tryAsJsCallback((param) => {
    return param;
});
```

#### `tryAsAsyncJsCallback()`

Allows asynchronous execution of Rust code in preparation of an asynchronous javascript callback. The result of the Rust preparation is passed as argument of the JavaScript lambda function that represents the callback. Before passing it through as response, Rust has the opportunity to intercept the callback return value

```javascript
let response = await module.tryAsJsCallback(async (param) => {
    return param;
});
```
