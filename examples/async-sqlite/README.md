# Async SQLite

The Async SQLite example implements a simple database in Rust with a JavaScript API.

## Usage

```js
const { Database } = require(".");

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

#### `#[neon::export(class)]`

The Rust sender side of the channel is held in a JavaScript [class][class]. It can be referenced later in methods.  

#### Rust and Neon Channels

The mpsc channel provides a way for the JavaScript main thread to communicate with the database thread, but it is one-way. In order to complete the callback, the database thread must be able to communicate with the JavaScript main thread. [`neon::event::Channel`][channel] provides a channel for sending these events back.

### JavaScript

[thread]: https://doc.rust-lang.org/std/thread/
[mpsc]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[class]: https://docs.rs/neon/latest/neon/attr.class.html
[channel]: https://docs.rs/neon/latest/neon/event/struct.Channel.html
[handle]: https://docs.rs/neon/latest/neon/handle/struct.Handle.html
