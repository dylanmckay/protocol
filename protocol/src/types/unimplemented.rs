
use {Parcel, Error, Settings};

use std::io::prelude::*;

/// A type that does not have any protocol serialization implemented.
///
/// Panics whenever a read or write of this value is attempted.
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

