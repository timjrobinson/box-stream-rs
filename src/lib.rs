//! Implementation of the [box-stream](https://github.com/dominictarr/pull-box-stream)
//! encryption protocol. This crate provides structs which wrap readers and/or
//! writers, decrypting all reads and encrypting all writes. Also provides
//! [tokio](https://tokio.rs/)'s asynchronous reade and write traits.

#![deny(missing_docs)]

extern crate libc;
extern crate sodiumoxide;
extern crate futures;
#[macro_use]
extern crate tokio_io;

pub mod crypto;
mod box_writer;
mod box_reader;
mod box_duplex;
mod decryptor;
mod encryptor;

pub use decryptor::{UNAUTHENTICATED_EOF, INVALID_LENGTH, UNAUTHENTICATED_HEADER,
                    UNAUTHENTICATED_PACKET};

pub use box_writer::*;
pub use box_reader::*;
pub use box_duplex::*;

#[cfg(test)]
extern crate partial_io;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
extern crate async_ringbuffer;
#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod test;
