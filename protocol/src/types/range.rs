use crate::{hint, Parcel, Error, Settings};
use std::io::prelude::*;

impl<T> Parcel for std::ops::Range<T>
    where T: Parcel {
    const TYPE_NAME: &'static str = "Range";

    fn read_field(read: &mut dyn Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let start = Parcel::read(read, settings)?;
        let end = Parcel::read(read, settings)?;

        Ok(std::ops::Range { start, end })
    }

    fn write_field(&self, write: &mut dyn Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.start.write(write, settings)?;
        self.end.write(write, settings)?;

        Ok(())
    }
}

