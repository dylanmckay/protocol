pub use self::numerics::Integer;
pub use self::array::Array;
pub use self::string::String;

pub mod numerics;
#[macro_use]
pub mod composite;
pub mod array;
pub mod map;
pub mod string;
pub mod tuple;
pub mod option;
pub mod vec_deque;
#[cfg(feature = "uuid")]
pub mod uuid;

use std::io::prelude::*;
use std::{fmt, io};

/// The default byte ordering.
pub type ByteOrder = ::byteorder::BigEndian;

/// A type which can be read or written.
pub trait Type : Clone + fmt::Debug
{
    /// Reads a type for a stream.
    fn read(read: &mut Read) -> Result<Self, ::Error>;

    /// Writes a type to a stream.
    fn write(&self, write: &mut Write) -> Result<(), ::Error>;

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, ::Error> {
        let mut buffer = ::std::io::Cursor::new(bytes);
        Self::read(&mut buffer)
    }

    fn raw_bytes(&self) -> Result<Vec<u8>, ::Error> {
        let mut buffer = io::Cursor::new(Vec::new());
        self.write(&mut buffer)?;

        Ok(buffer.into_inner())
    }
}

