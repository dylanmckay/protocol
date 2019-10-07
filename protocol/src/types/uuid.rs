use {Parcel, Error, Settings};
use hint;
use std::io::prelude::*;

use uuid::Uuid;

impl Parcel for Uuid
{
    const TYPE_NAME: &'static str = "Uuid";

    fn read_field(read: &mut dyn Read,
                  settings: &Settings,
                  _: &mut hint::Hints)
        -> Result<Self, Error> {
        let bytes: [u8; 16] = Parcel::read(read, settings)?;

        Ok(Uuid::from_bytes(bytes))
    }

    fn write_field(&self,
                   write: &mut dyn Write,
                   _: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        write.write(self.as_bytes())?;
        Ok(())
    }
}

