use {Parcel, Error, Settings};
use hint;
use std::io::prelude::*;

use uuid::Uuid;

impl Parcel for Uuid
{
    const TYPE_NAME: &'static str = "Uuid";

    fn read_field(read: &mut Read,
                  _: &Settings,
                  _: &mut hint::Hints)
        -> Result<Self, Error> {
        let bytes: Result<Vec<u8>, _> = read.bytes().take(16).collect();
        let bytes = bytes?;

        Ok(Uuid::from_bytes(&bytes)?)
    }

    fn write_field(&self,
                   write: &mut Write,
                   _: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        write.write(self.as_bytes())?;
        Ok(())
    }
}

