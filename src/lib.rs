#![feature(question_mark)]

#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

pub use self::types::*;
pub use self::error::Error;
pub use self::packet::{PacketKind, Packet};

#[macro_use]
pub mod types;
#[macro_use]
pub mod packet;
pub mod error;
#[macro_use]
pub mod wire;

extern crate byteorder;

