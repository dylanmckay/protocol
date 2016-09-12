use {Type, Error};
use std::io::prelude::*;

use uuid::Uuid;

impl Type for Uuid
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let bytes: Result<Vec<u8>, _> = read.bytes().take(16).collect();
        let bytes = bytes?;

        Ok(Uuid::from_bytes(&bytes)?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write(self.as_bytes())?;
        Ok(())
    }
}

