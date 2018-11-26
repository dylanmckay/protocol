use {Parcel, Error, Settings};
use hint;
use std::marker::PhantomData;

use std::io::prelude::*;

impl<T> Parcel for PhantomData<T>
{
    const TYPE_NAME: &'static str = "PhantomData<T>";

    fn read(_: &mut Read,
            _: &Settings,
            _: &mut hint::Hints) -> Result<Self, Error> {
        Ok(PhantomData)
    }

    fn write(&self, _: &mut Write,
             _: &Settings) -> Result<(), Error> {
        Ok(())
    }
}

