use crate::{Parcel, Settings, Error};
use crate::hint;

use std::rc::Rc;
use std::sync::Arc;
use std::ops::Deref;
use std::io::prelude::*;

macro_rules! impl_smart_ptr_type {
    ($ty:ident) => {
        impl<T: Parcel> Parcel for $ty<T>
        {
            const TYPE_NAME: &'static str = stringify!($ty<T>);

            fn read_field(read: &mut Read,
                          settings: &Settings,
                          _: &mut hint::Hints) -> Result<Self, Error> {
                let value = T::read(read, settings)?;
                Ok($ty::new(value))
            }

            fn write_field(&self, write: &mut Write,
                           settings: &Settings,
                           _: &mut hint::Hints) -> Result<(), Error> {
                self.deref().write(write, settings)
            }
        }
    }
}

impl_smart_ptr_type!(Rc);
impl_smart_ptr_type!(Arc);

