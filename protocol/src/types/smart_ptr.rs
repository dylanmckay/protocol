use crate::{hint, Parcel, Settings, Error};

use std::rc::Rc;
use std::sync::Arc;
use std::ops::Deref;
use std::io::prelude::*;

macro_rules! impl_smart_ptr_type {
    ($ty:ident) => {
        impl<T: Parcel> Parcel for $ty<T>
        {
            const TYPE_NAME: &'static str = stringify!($ty<T>);

            fn read_field(read: &mut dyn Read,
                          settings: &Settings,
                          _: &mut hint::Hints) -> Result<Self, Error> {
                let value = T::read(read, settings)?;
                Ok($ty::new(value))
            }

            fn write_field(&self, write: &mut dyn Write,
                           settings: &Settings,
                           _: &mut hint::Hints) -> Result<(), Error> {
                self.deref().write(write, settings)
            }
        }
    }
}

impl_smart_ptr_type!(Rc);
impl_smart_ptr_type!(Arc);

#[cfg(all(feature = "high-level-trait", feature = "impl-box"))]
compile_error!("it is not possible to enable the Parcel impl for Box<T> while the 'high-level-trait' feature flag is enabled, consider removing it as it is slated to be deprecated anyway");

// The high level trait's blanket impl conflicts with
// any Parcel implementation for Box<T>.
#[cfg(all(feature = "impl-box", not(feature = "high-level-trait")))]
impl_smart_ptr_type!(Box); // technically not a smart ptr, but hardly matters
