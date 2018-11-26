use {Parcel, Settings, Error};
use hint;

use std::rc::Rc;
use std::sync::Arc;
use std::ops::Deref;
use std::io::prelude::*;

macro_rules! impl_smart_ptr_type {
    ($ty:ident) => {
        impl<T: Parcel> Parcel for $ty<T>
        {
            const TYPE_NAME: &'static str = stringify!($ty<T>);

            fn read(read: &mut Read,
                    settings: &Settings,
                    hints: &mut hint::Hints) -> Result<Self, Error> {
                let value = T::read(read, settings, hints)?;
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

