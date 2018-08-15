//! Contains newtypes over the standard library types
//! that support finer-grained serialization settings.

pub use self::numerics::Integer;
pub use self::string::String;
pub use self::vec::Vec;

mod numerics;
#[macro_use]
mod composite;
mod array;
mod string;
mod char;
mod tuple;
mod option;
/// Definitions for the `std::collections` module.
mod collections;

mod util;

#[cfg(feature = "uuid")]
mod uuid;
mod vec;

