use crate::{hint, Parcel, Error, CharTryFromError, Settings};
use std::char;
use std::io::prelude::*;

impl Parcel for char
{
    const TYPE_NAME: &'static str = "char";

    fn read_field(read: &mut dyn Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let bytes = u32::read(read, settings)?;
        Ok(char::from_u32(bytes).ok_or(CharTryFromError{ })?)
    }

    fn write_field(&self, write: &mut dyn Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        (*self as u32).write(write, settings)
    }
}

