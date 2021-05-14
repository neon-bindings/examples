# Async SQLite

The Async SQLite example implements a simple database in Rust with a JavaScript API.

## Usage

```js
const Database = require(".");

(async () => {
    const db = new Database();

    const id = await db.insert("Marty McFly");
    const name = await db.byId(id);

    console.log(name);
})();
```

## Design

### Rust

SQLite provides a _synchronous_ interface. This means that the current thread is blocked while a query is executing. Ideally, JavaScript would be able to continue executing concurrently with query execution.

The Async SQLite example demonstrates one pattern for moving database operations to a separate thread and asynchronously calling back to JavaScript when the operation has completed.

#### Threads and Channels

Since SQLite is naturally single threaded, our application does not benefit from a thread pool or connection pool when querying the database. Instead, a _single_ rust [thread][thread] is spawned for performing database operations.

Once the database thread is spawned, the JavaScript main thread needs a way to communicate with it. A [multi-producer, single-consumer (mpsc)][mpsc] channel is created. The receiving end is owned by the database thread and the sender is held by JavaScript.

#### `JsBox`

Rust data cannot be directly held by JavaScript. The [`JsBox`][jsbox] provides a mechanism for allowing JavaScript to hold a reference to Rust data and later access it again from Rust.

#### Channels and `EventQueue`

The mpsc channel provides a way for the JavaScript main thread to communicate with the database thread, but it is one-way. In order to complete the callback, the database thread must be able to communicate with the JavaScript main thread. [`EventQueue`][eventqueue] provides a channel for sending these events back.

#### `Root`

The last issue to solve is sending a reference to the JavaScript callback to the database thread and back again before finally calling it. [Handles][handle] to JavaScript values are not `Send`; they cannot escape the scope that created them. The reason they cannot be passed to other threads is because when control is returned back to the JavaScript engine, the garbage collector may determine they are no longer used and free the value.

A [`Root`][root] is a special handle to a JavaScript value that prevents the value from being freed as long as the `Root` has not been dropped. By placing the callback in a `Root`, it can be safely sent across threads and finally accessed and called when back on the JavaScript main thread.

### JavaScript

[thread]: https://doc.rust-lang.org/std/thread/
[mpsc]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[jsbox]: https://docs.rs/neon/0.8.1-napi/neon/types/struct.JsBox.html
[eventqueue]: https://docs.rs/neon/0.8.1-napi/neon/event/struct.EventQueue.html
[handle]: https://docs.rs/neon/0.8.1-napi/neon/handle/struct.Handle.html
[root]: https://docs.rs/neon/0.8.1-napi/neon/handle/struct.Root.html
