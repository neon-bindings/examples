use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};

use flate2::{Compression, write::GzEncoder};
use neon::prelude::*;
use neon::types::{JsUint8Array, buffer::TypedArray};
use std::fmt::Debug;

#[derive(Clone)]
// Holds state for a gzip compression stream
struct CompressStream {
    // The stream will be held by multiple threads
    // * `Arc` ensures the encoder won't be garbage collected while a chunk
    //   is being processed
    // * `Mutex` gives mutable access to the encoder. Even though only a single
    //   thread _should_ be accessing the encoder, multiple calls to `compressChunk`
    //   could cause contention.
    encoder: Arc<Mutex<GzEncoder<Vec<u8>>>>,
}

impl CompressStream {
    // Create a new instance of a `CompressStream`
    fn new(level: Compression) -> Self {
        let encoder = GzEncoder::new(Vec::new(), level);

        Self {
            encoder: Arc::new(Mutex::new(encoder)),
        }
    }

    // Attempt to obtain a mutable reference to the stream
    fn lock(&self) -> Result<MutexGuard<'_, GzEncoder<Vec<u8>>>, CompressError> {
        // Use `try_lock` instead of `lock` because multiple concurrent calls
        // to the encoder is undefined. The caller *must* be careful to serially
        // write to the stream. `Transform` provides this guarantee by applying
        // backpressure and buffering. The `Mutex` should always be unlocked.
        Ok(self.encoder.try_lock()?)
    }

    // Write a chunk of data to the encoder
    fn write(self, data: Vec<u8>) -> Result<Self, CompressError> {
        self.lock()?.write_all(&data)?;
        Ok(self)
    }

    // Finish compressing. Multiple calls to this function will error or panic.
    fn finish(self) -> Result<Self, CompressError> {
        self.lock()?.try_finish()?;
        Ok(self)
    }

    // After each call to `write` or `finish`, data may be written to the internal
    // buffer. This function copies the written data out and resets the buffer
    // to empty.
    fn and_buffer(
        mut cx: TaskContext,
        // Return value from `cx.task(..)` closure
        result: Result<Self, CompressError>,
    ) -> JsResult<JsUint8Array> {
        let stream = result.or_else(|err| cx.throw_error(err))?;
        let mut guard = stream.lock().or_else(|err| cx.throw_error(err))?;

        let data = guard.get_mut();
        let output = JsUint8Array::from_slice(&mut cx, data)?;

        data.truncate(0);

        Ok(output)
    }
}

// Types placed in a `JsBox`, an opaque pointer for passing Rust data from Rust to
// JavaScript and back, must implement the `Finalize` trait.
//
// The `Finalize` trait optionally provides a hook for executing code when the value
// is garbage collected.
impl Finalize for CompressStream {}

#[derive(Debug)]
// All errors will be converted to JavaScript exceptions with the `Display`
// implementation as the `Error` message.
struct CompressError(String);

impl From<io::Error> for CompressError {
    fn from(err: io::Error) -> Self {
        Self(err.to_string())
    }
}

impl<T> From<TryLockError<T>> for CompressError {
    fn from(err: TryLockError<T>) -> Self {
        Self(err.to_string())
    }
}

impl fmt::Display for CompressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for CompressError {}

impl AsRef<str> for CompressError {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Create a boxed `CompressStream` that can be passed to JavaScript and back
fn compress_new(mut cx: FunctionContext) -> JsResult<JsBox<CompressStream>> {
    // Best compression because why not?
    let stream = CompressStream::new(Compression::best());

    // `cx.boxed` creates an opaque pointer that can cross the FFI boundary
    Ok(cx.boxed(stream))
}

// Compress a chunk of data on the Node worker thread pool, returning a promise
// that may contain compressed data.
fn compress_chunk(mut cx: FunctionContext) -> JsResult<JsPromise> {
    // This is some funky syntax, but it's taking the first argument to the function,
    // attempting to downcast it as a `JsBox<CompressStream>` and finally calling
    // the `Clone` implementation on `CompressStream`. The `&**` is due to a couple
    // of smart pointers. Smart pointers are types that implement the
    // [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) trait.
    //
    // The outer type is `neon::handle::Handle`. `Handle` is a type used by Neon for
    // ensuring that references to JavaScript values cannot be held after they have
    // been garbage collected. The next type is `JsBox` which is a smart pointer for
    // holding a reference to Rust data in JavaScript. The `**` dereferences these two
    // types, giving a `CompressStream`. However, it's impossible to move out of a
    // `JsBox`, so a reference is immediately taken with `&`. Finally, we can call the
    // `clone` implementation on `CompressStream`.
    let stream = (**cx.argument::<JsBox<CompressStream>>(0)?).clone();

    // The 2nd argument is `encoding`. However, gzip is encoding agnostic and we do not need it.
    // let encoding = cx.argument::<JsString>(1)?;

    // Grab the 3rd argument as a `Uint8Array`. The data is immediately converted to a
    // `Vec<u8>` by borrowing as a `&[u8]` and cloning.
    let chunk = cx.argument::<JsTypedArray<u8>>(2)?.as_slice(&cx).to_vec();

    let promise = cx
        // Create a task to execute on the Node worker pool
        .task(move || stream.write(chunk))
        // Convert the result of the task into an `ArrayBuffer` and resolve the promise
        // on the JavaScript main thread.
        .promise(CompressStream::and_buffer);

    Ok(promise)
}

// Complete compressing the data and get the remaining output
fn compress_finish(mut cx: FunctionContext) -> JsResult<JsPromise> {
    // Get a shallow clone of `CompressStream`; same as in `compress_chunk`
    // This is an alternative to the `**` syntax used earlier. Instead, it uses auto-deref
    // and universal call syntax for the `clone` call to coerce to proper type.
    let stream = CompressStream::clone(&*cx.argument::<JsBox<CompressStream>>(0)?);

    let promise = cx
        // Finish the stream on the Node worker pool
        .task(move || stream.finish())
        // Convert the remaining output into an `ArrayBuffer` and resolve the promise
        // on the JavaScript main thread.
        .promise(CompressStream::and_buffer);

    Ok(promise)
}

#[neon::main]
// Called once when the module is loaded
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // Export each of the Neon functions as part of the module
    cx.export_function("compressNew", compress_new)?;
    cx.export_function("compressChunk", compress_chunk)?;
    cx.export_function("compressFinish", compress_finish)?;

    Ok(())
}
