// Implementation of BoxReader, a wrapper for Readers that decrypts all reads.

use futures_core::Poll;
use futures_core::task::Context;
use futures_io::{Error, AsyncRead};
use sodiumoxide::crypto::secretbox;

use decryptor::*;

/// Wraps a reader, decrypting all reads.
pub struct BoxReader<R> {
    inner: R,
    key: secretbox::Key,
    nonce: secretbox::Nonce,
    decryptor: Decryptor,
}

impl<R> BoxReader<R> {
    /// Create a new reader, wrapping `inner` and using `key` and `nonce` for
    /// decryption.
    pub fn new(inner: R, key: secretbox::Key, nonce: secretbox::Nonce) -> BoxReader<R> {
        BoxReader {
            inner,
            key,
            nonce,
            decryptor: Decryptor::new(),
        }
    }

    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    ///
    /// It is inadvisable to directly write to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Unwraps this `BoxReader`, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: AsyncRead> AsyncRead for BoxReader<R> {
    /// Read bytes from the wrapped reader and decrypt them. End of stream is signalled by
    /// returning `Ok(0)` even though this function was passed a buffer of nonzero length.
    ///
    /// # Errors
    /// In addition to propagating all errors from the wrapped reader, a
    /// `BoxReader` produces the following error kinds:
    ///
    /// - `ErrorKind::InvalidData`: If data could not be decrypted, or if a
    /// header declares an invalid length. Possible error values are
    /// `INVALID_LENGTH`, `UNAUTHENTICATED_HEADER`, `UNAUTHENTICATED_PACKET`.
    /// `ErrorKind::UnexpectedEof`: If a call to the inner reader returned `Ok(0)` although it was
    /// given a buffer of nonzero length. This is an error since end of file must be signalled via
    /// a special header in a box stream. The error value for this is `UNAUTHENTICATED_EOF`.
    fn poll_read(&mut self, cx: &mut Context, buf: &mut [u8]) -> Poll<usize, Error> {
        self.decryptor
            .poll_read(cx, buf, &mut self.inner, &self.key, &mut self.nonce)
    }
}
