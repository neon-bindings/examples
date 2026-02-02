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
    const stream = new CompressStream();

    return new Transform({
        transform(chunk, encoding, callback) {
            stream
                .compress(encoding, chunk)
                .then(data => callback(null, data))
                .catch(callback);
        },

        flush(callback) {
            stream
                .finish()
                .then(data => callback(null, data))
                .catch(callback);
        }
    });
}
```

The glue code exports a single function `compress` responsible for creating a `Transform` stream, delegating to the `CompressStream` class and adapting [`Promise`s](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) to callbacks.  

## Neon

The Neon module exports a `CompressSteam` class with two methods:

* [`compress`](#compresschunk-encoding)
* [`finish`](#finish)

### `new CompressStream(level)`

Creates a new instance of the `CompressStream` class with an optional gzip level.

### `compress(chunk, encoding)`

Compresses a chunk, returning flushed data as an `ArrayBuffer`.

### `finish()`

`finish` works very similar to [`compress`](#compresschunk-encoding), except it is provided the arguments to [`flush`](#flushcallback) which does not include any data. Instead, the remaining buffered data is compressed, a CRC is calculated, and the compressed gzip data is completed.
