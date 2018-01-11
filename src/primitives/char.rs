use {Parcel, Error, CharTryFromError};

use std::char;
use std::io::prelude::*;

impl Parcel for char
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let bytes = u32::read(read)?;
        Ok(char::from_u32(bytes).ok_or(CharTryFromError{ })?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        (*self as u32).write(write)
    }
}

