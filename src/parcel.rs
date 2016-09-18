use std::io::prelude::*;
use std::{fmt, io};

/// A value which can be read and written.
pub trait Parcel : Clone + fmt::Debug + PartialEq
{
    /// Reads a value from a stream.
    fn read(read: &mut Read) -> Result<Self, ::Error>;

    /// Writes a value to a stream.
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

