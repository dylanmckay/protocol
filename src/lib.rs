#![feature(try_from)]

//! Simple packet-based protocol definitions in Rust.
//!
//! * The `packet` module deals with defining packets.
//! * The `wire` module deals with transmission of packets.

pub use self::primitives::{Integer, DynArray, String};
pub use self::parcel::Parcel;
pub use self::errors::{Error, ErrorKind, ResultExt};

// Must go first because it defines common macros.
#[macro_use]
mod packet;

#[macro_use]
pub mod primitives;
#[macro_use]
pub mod wire;

mod errors;
mod parcel;

extern crate byteorder;
extern crate flate2;
#[macro_use]
extern crate error_chain;

#[cfg(feature = "uuid")]
extern crate uuid;

/// The default byte ordering.
pub type ByteOrder = ::byteorder::BigEndian;

