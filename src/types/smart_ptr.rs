use {Type, Error};

use std::rc::Rc;
use std::sync::Arc;
use std::ops::Deref;
use std::io::prelude::*;

macro_rules! impl_smart_ptr_type {
    ($ty:ident) => {
        impl<T: Type> Type for $ty<T>
        {
            fn read(read: &mut Read) -> Result<Self, Error> {
                let value = T::read(read)?;
                Ok($ty::new(value))
            }

            fn write(&self, write: &mut Write) -> Result<(), Error> {
                self.deref().write(write)
            }
        }
    }
}

impl_smart_ptr_type!(Rc);
impl_smart_ptr_type!(Arc);

