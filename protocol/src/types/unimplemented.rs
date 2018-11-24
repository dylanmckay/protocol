
use {Parcel, Error, Settings};

use std::io::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unimplemented;

impl Parcel for Unimplemented
{
    const TYPE_NAME: &'static str = "Unimplemented";

    fn read(_: &mut Read,
            _: &Settings) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write(&self, _: &mut Write,
             _: &Settings) -> Result<(), Error> {
        unimplemented!();
    }
}

