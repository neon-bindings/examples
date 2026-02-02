use std::{
    io::Write,
    sync::{Arc, Mutex, MutexGuard},
};

use flate2::{Compression, write::GzEncoder};
use neon::types::extract::Error;

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

#[neon::export(class)]
impl CompressStream {
    fn new(level: Option<u32>) -> Self {
        let level = Compression::new(level.unwrap_or(9));
        let encoder = GzEncoder::new(Vec::new(), level);

        Self {
            encoder: Arc::new(Mutex::new(encoder)),
        }
    }

    #[neon(task)]
    fn compress(self, _encoding: (), chunk: Vec<u8>) -> Result<Vec<u8>, Error> {
        let mut guard = self.lock()?;
        guard.write_all(&chunk)?;
        Ok(next_chunk(&mut guard))
    }

    #[neon(task)]
    fn finish(self) -> Result<Vec<u8>, Error> {
        let mut guard = self.lock()?;
        guard.try_finish()?;
        Ok(next_chunk(&mut guard))
    }
}

// Rust allows multiple `impl` blocks. Rust methods that should not be exposed to JavaScript
// can be written here.
impl CompressStream {
    // Attempt to obtain a mutable reference to the stream
    fn lock(&self) -> Result<MutexGuard<'_, GzEncoder<Vec<u8>>>, Error> {
        // Use `try_lock` instead of `lock` because multiple concurrent calls
        // to the encoder is undefined. The caller *must* be careful to serially
        // write to the stream. `Transform` provides this guarantee by applying
        // backpressure and buffering. The `Mutex` should always be unlocked.
        self.encoder
            .try_lock()
            .map_err(|_| Error::new("GzEncoder already locked"))
    }
}

fn next_chunk(encoder: &mut GzEncoder<Vec<u8>>) -> Vec<u8> {
    let chunk = encoder.get_mut().clone();
    encoder.get_mut().truncate(0);
    chunk
}
