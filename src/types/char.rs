use {Type, Error};

use std::io::prelude::*;

impl Type for char
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        use std::convert::TryFrom;

        let bytes = u32::read(read)?;
        Ok(char::try_from(bytes)?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        (*self as u32).write(write)
    }
}

