//! Contains newtypes over the standard library types
//! that support finer-grained serialization settings.

pub use crate::types::{
    numerics::Integer, string::String, unimplemented::Unimplemented, vec::Vec
};
mod array;
mod char;
/// Definitions for the `std::collections` module.
mod collections;
mod marker;
mod numerics;
mod option;
mod string;
mod tuple;
/// Definitions for smart pointers in the `std` module.
mod smart_ptr;
mod unimplemented;
#[cfg(feature = "uuid")]
mod uuid;
mod vec;

