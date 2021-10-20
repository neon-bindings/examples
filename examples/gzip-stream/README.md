# Async gzip compress

The gzip compression example demonstrates building a ["through" stream](https://nodejs.dev/learn/nodejs-streams) with Neon. The stream is both readable and writeable and CPU intensive processing occurs on the Node worker thread pool.

## Design

A small amount of JavaScript glue code can often simplify Neon modules. In this case, Node.js already provides a [`Transform`](https://nodejs.org/api/stream.html#stream_duplex_and_transform_streams) stream class. The `Transform` stream provides important features that could be complex to implement:

* Writeable
* Readable
* Backpressure
* Asynchronous

Two methods must be implemented.

### `transform(chunk, encoding, callback)`

The `transform` method accepts a chunk of data and its encoding, as well a callback. The callback should be called when processing the `chunk` has completed.

### `flush(callback)`

The `flush` method allows any internally buffered data to be processed before completion. The `callback` is identical to the callback in `transform`.

## Glue

```js
function compress() {
    const compressor = compressNew();

    return new Transform({
        transform(chunk, encoding, callback) {
            compressChunk(compressor, encoding, chunk)
                .then(data => callback(null, data))
                .catch(callback);
        },

        flush(callback) {
            compressFinish(compressor)
                .then(data => callback(null, data))
                .catch(callback);
        }
    });
}
```

The glue code exports a single function `compress` that creates a `Transform` stream delegating the implementation to Neon functions. Since these functions return promises, they are adapted to the `callback` style continuation that `Transform` expects.

## Neon

The Neon module exports three functions:

* [`compressNew`](#compressnew)
* [`compressChunk`](#compresschunkcompressstream-chunk-encoding-callback)
* [`compressFinish`](#compressfinishcompressstream-callback)

### `compressNew()`

```rust
fn compress_new(mut cx: FunctionContext) -> JsResult<JsBox<CompressStream>> {
    let stream = CompressStream::new(Compression::best());

    Ok(cx.boxed(stream))
}
```

`compressNew` creates an instance of the stateful Rust struct, `CompressStream`, and returns it wrapped in a [`JsBox`](https://docs.rs/neon/latest/neon/types/struct.JsBox.html). Each of the other two methods expects `CompressStream` as the first argument. This pattern is similar to using [`Function.prototype.call`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/call) on a class method to manually bind `this`.

### `compressChunk(compressStream, chunk, encoding, callback)`

```rust
fn compress_chunk(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let stream = (&**cx.argument::<JsBox<CompressStream>>(0)?).clone();
    let chunk = cx.argument::<JsTypedArray<u8>>(2)?
        .as_slice(&cx)
        .to_vec();

    let promise = cx
        .task(move || stream.write(chunk))
        .promise(CompressStream::and_buffer);

    Ok(promise)
}
```

`compressChunk` accepts the instance of the `CompressStream` struct and the other arguments to the [`transform`](#transformchunk-encoding-callback) function. The chunk is cloned to a `Vec<u8>` and passed to a task to execute on the Node worker pool. The asynchronous task compresses the data and passes the compressed data to the `.promise(|cx, result| { ... })` callback. The callback to `promise` is executed on the JavaScript main thread and converts the compressed `Vec<u8>` to a `JsBuffer` and resolves the promise.

`CompressChunk::and_buffer` is used to create a `Buffer`. `ArrayBuffer` cannot be used because stream chunks are required to be an instance of `Uint8Array`. `Buffer` is a subclass of `Uint8Array`.

### `compressFinish(compressStream, callback)`

fn compress_finish(mut cx: FunctionContext) -> JsResult<JsPromise> {
let stream = (&**cx.argument::<JsBox<CompressStream>>(0)?).clone();

```rust
fn compress_finish(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let stream = (&**cx.argument::<JsBox<CompressStream>>(0)?).clone();

    let promise = cx
        .task(move || stream.finish())
        .promise(CompressStream::and_buffer);

    Ok(promise)
}
```

`compressFinish` works very similar to [`compressChunkl`](#compresschunkcompressstream-chunk-encoding-callback), except it is provided the arguments to [`flush`](#flushcallback) which does not include any data. Instead, the remaining buffered data is compressed, a CRC is calculated, and the compressed gzip data is completed.
