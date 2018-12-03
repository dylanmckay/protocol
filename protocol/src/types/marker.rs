use {Parcel, Error, Settings};
use hint;
use std::marker::PhantomData;

use std::io::prelude::*;

impl<T> Parcel for PhantomData<T>
{
    const TYPE_NAME: &'static str = "PhantomData<T>";

    fn read_field(_: &mut Read,
                  _: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        Ok(PhantomData)
    }

    fn write_field(&self,
                   _: &mut Write,
                   _: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        Ok(())
    }
}

