# `tokio-fetch`

Example of spawning a Rust async task on the [tokio][tokio] thread pool and resolving a JavaScript [Promise][promise] after it completes.

_**Note:** This example uses a pre-release version of Neon._

[tokio]: https://tokio.rs
[promise]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise

## Methods

#### `function nodeReleaseDate(): Promise<string>`

Asynchronously fetch the release date for the currently running Node process from nodejs.org.

## Design

### Executor

For optimum task scheduling, it is best to have a single Rust task executor (e.g., tokio runtime). To make the runtime singleton available to Neon functions, it is stored in a global using `OnceCell`.

```rust
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

static RUNTIME: OnceCell<Runtime> = OnceCell::new();
```

A small helper is provided to lazily initialize the runtime and throw an exception on failure.

```rust
fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}
```

### Spawning Tasks

Tasks may be spawned on the tokio runtime by using the `RUNTIME` handle. Spawning a task does *not* block the current thread. Inside a task the `await` keyword may be used and typical async Rust patterns may be used.

```rust
let rt = runtime(&mut cx)?;

rt.spawn(async move {
    // Asynchronous Rust may used in here
});
```

### Promises

When a task is spawned on the tokio runtime, it will be executed at a later time. JavaScript needs to be notified when the task completes. 

* Neon [`Channel`][channel] may be created for moving an operation from the tokio thread pool back to the JavaScript main thread.
* [`cx.promise()`][cx-promise] creates a [`JsPromise`][js-promise] and [`Deferred`][deferred] for signaling JavaScript.
* [`JsPromise`][js-promise] is synchronously returned and may be used with `await` in JavaScript
* [`Deferred`][deferred] is used to settle the [`JsPromise`][js-promise] from the [`Channel`][channel] callback.

```rust
let channel = cx.channel();
let (deferred, promise) = cx.promise();

rt.spawn(async move {
    // Code here executes non-blocking on the tokio thread pool

    deferred.settle_with(&channel, move |mut cx| {
        // Code here executes blocking on the JavaScript main thread

        Ok(cx.undefined())
    });
});

Ok(promise)
```

[channel]: https://docs.rs/neon/0.10.0-alpha.3/neon/event/struct.Channel.html
[cx-promise]: https://docs.rs/neon/0.10.0-alpha.3/neon/context/trait.Context.html#method.promise
[js-promise]: https://docs.rs/neon/0.10.0-alpha.3/neon/types/struct.JsPromise.html
[deferred]: https://docs.rs/neon/0.10.0-alpha.3/neon/types/struct.Deferred.html
