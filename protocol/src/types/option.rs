use {Parcel, Error, Settings};

use std::io::prelude::*;

impl<T: Parcel> Parcel for Option<T>
{
    const TYPE_NAME: &'static str = "Option<T>";

    fn read(read: &mut Read,
            settings: &Settings) -> Result<Self, Error> {
        let is_some = bool::read(read, settings)?;

        if is_some {
            let value = T::read(read, settings)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn write(&self, write: &mut Write,
             settings: &Settings) -> Result<(), Error> {
        self.is_some().write(write, settings)?;

        if let Some(ref value) = *self {
            value.write(write, settings)?;
        }

        Ok(())
    }
}

