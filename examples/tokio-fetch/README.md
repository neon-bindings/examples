# `tokio-fetch`

Example of spawning a Rust async task on the [tokio][tokio] thread pool and resolving a JavaScript [Promise][promise] after it completes.

[tokio]: https://tokio.rs
[promise]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise

## Methods

### `function nodeReleaseDate(version: string): Promise<string>`

Asynchronously fetch the release date for a given Node release from nodejs.org.

`nodeReleaseDate` is a Rust `async fn`. The argument uses [`TryFromJs`][tryfromjs] and return value uses [`TryIntoJs`][tryintojs] for ergonomic conversions.  

### `function currentNodeReleaseDate(): Promise<string>`

Asynchronously fetch the release date for the currently running Node process from nodejs.org.

`currentNodeReleaseDate` needs to access the JavaScript VM synchronously in order to get the current Node version before spawning an async tokio task.

Writing a _synchronous_ Rust `fn`, that returns a Rust future, allows synchronous setup code that uses [`Cx`][cx].

**Note**: The returned future is still required to be `Send + 'static`.

[tryfromjs]: https://docs.rs/neon/latest/neon/types/extract/trait.TryFromJs.html
[tryintojs]: https://docs.rs/neon/latest/neon/types/extract/trait.TryIntoJs.html
[cx]: https://docs.rs/neon/latest/neon/context/struct.Cx.html
