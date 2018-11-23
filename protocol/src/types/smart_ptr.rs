use {Parcel, Settings, Error};

use std::rc::Rc;
use std::sync::Arc;
use std::ops::Deref;
use std::io::prelude::*;

macro_rules! impl_smart_ptr_type {
    ($ty:ident) => {
        impl<T: Parcel> Parcel for $ty<T>
        {
            const TYPE_NAME: &'static str = stringify!($ty<T>);

            fn read(read: &mut Read, settings: &Settings) -> Result<Self, Error> {
                let value = T::read(read, settings)?;
                Ok($ty::new(value))
            }

            fn write(&self, write: &mut Write,
                     settings: &Settings) -> Result<(), Error> {
                self.deref().write(write, settings)
            }
        }
    }
}

impl_smart_ptr_type!(Rc);
impl_smart_ptr_type!(Arc);

