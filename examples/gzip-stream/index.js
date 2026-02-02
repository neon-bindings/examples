"use strict";

// Transform stream reduces the boilerplate of a stream that reads bytes from a
// source and produces new bytes for a destination sink.
const { Transform } = require("stream");

const { CompressStream } = require("./index.node");

// Creates a gzip compression transform stream, implemented asynchronously in Rust
function compress() {
    // Create a native streaming gzip compressing with Neon
    const stream = new CompressStream();

    return new Transform({
        // Compress a chunk of data by delegating to `compressChunk`
        transform(chunk, encoding, callback) {
            stream
                .compress(encoding, chunk)
                .then(data => callback(null, data))
                .catch(callback);
        },

        // Complete the compression by delegating to `compressFinish`
        flush(callback) {
            stream
                .finish()
                .then(data => callback(null, data))
                .catch(callback);
        }
    });
}

module.exports = compress;
