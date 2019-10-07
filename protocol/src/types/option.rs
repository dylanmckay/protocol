use {Parcel, Error, Settings};
use hint;

use std::io::prelude::*;

impl<T: Parcel> Parcel for Option<T>
{
    const TYPE_NAME: &'static str = "Option<T>";

    fn read_field(read: &mut dyn Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let is_some = bool::read(read, settings)?;

        if is_some {
            let value = T::read(read, settings)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn write_field(&self, write: &mut dyn Write,
             settings: &Settings,
             _: &mut hint::Hints) -> Result<(), Error> {
        self.is_some().write(write, settings)?;

        if let Some(ref value) = *self {
            value.write(write, settings)?;
        }

        Ok(())
    }
}

