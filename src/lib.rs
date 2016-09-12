#![feature(question_mark)]
#![feature(try_from)]
#![feature(plugin)]

#![cfg_attr(test, plugin(stainless))]

//! Simple packet-based protocol definitions in Rust.
//!
//! * The `packet` module deals with defining packets.
//! * The `wire` module deals with transmission of packets.

pub use self::types::{Type, Integer, Array, String, ByteOrder};
pub use self::error::Error;
pub use self::packet::Packet;

#[macro_use]
pub mod types;
#[macro_use]
pub mod packet;
pub mod error;
#[macro_use]
pub mod wire;

extern crate byteorder;
extern crate flate2;

#[cfg(feature = "uuid")]
extern crate uuid;

