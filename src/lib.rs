#![feature(try_from)]

#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(stainless))]

//! Simple packet-based protocol definitions in Rust.
//!
//! * The `packet` module deals with defining packets.
//! * The `wire` module deals with transmission of packets.

pub use self::primitives::{Integer, Array, String};
pub use self::parcel::Parcel;
pub use self::error::Error;

pub mod parcel;
#[macro_use]
pub mod primitives;
#[macro_use]
pub mod packet;
pub mod error;
#[macro_use]
pub mod wire;

extern crate byteorder;
extern crate flate2;

#[cfg(feature = "uuid")]
extern crate uuid;

/// The default byte ordering.
pub type ByteOrder = ::byteorder::BigEndian;

