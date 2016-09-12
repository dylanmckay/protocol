use {Type, Error};

use std::rc::Rc;
use std::ops::Deref;
use std::io::prelude::*;

impl<T: Type> Type for Rc<T>
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let value = T::read(read)?;
        Ok(Rc::new(value))
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        self.deref().write(write)
    }
}

