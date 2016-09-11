use {Type, Error};
use std::io::prelude::*;

impl Type for String
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let bytes = Vec::<u8>::read(read)?;

        Ok(String::from_utf8(bytes)?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        let bytes: Vec<u8> = self.bytes().collect();
        bytes.write(write)
    }
}

