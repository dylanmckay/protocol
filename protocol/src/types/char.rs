use {Parcel, Error, CharTryFromError, Settings};
use hint;
use std::char;
use std::io::prelude::*;

impl Parcel for char
{
    const TYPE_NAME: &'static str = "char";

    fn read(read: &mut Read,
            settings: &Settings,
            hints: &mut hint::Hints) -> Result<Self, Error> {
        let bytes = u32::read(read, settings, hints)?;
        Ok(char::from_u32(bytes).ok_or(CharTryFromError{ })?)
    }

    fn write(&self, write: &mut Write,
             settings: &Settings) -> Result<(), Error> {
        (*self as u32).write(write, settings)
    }
}

