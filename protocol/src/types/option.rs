use {Parcel, Error, Settings};
use hint;

use std::io::prelude::*;

impl<T: Parcel> Parcel for Option<T>
{
    const TYPE_NAME: &'static str = "Option<T>";

    fn read(read: &mut Read,
            settings: &Settings,
            hints: &mut hint::Hints) -> Result<Self, Error> {
        let is_some = bool::read(read, settings, hints)?;

        if is_some {
            let value = T::read(read, settings, hints)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn write(&self, write: &mut Write,
             settings: &Settings,
             hints: &mut hint::Hints) -> Result<(), Error> {
        self.is_some().write(write, settings, &mut hint::Hints::default())?;

        if let Some(ref value) = *self {
            value.write(write, settings, hints)?;
        }

        Ok(())
    }
}

