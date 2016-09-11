use {Type, Error};

use std::io::prelude::*;

pub type SizeType = u32;

impl<T: Type> Type for Vec<T>
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let size = SizeType::read(read)?;

        let mut elements = Vec::new();

        for _ in 0..size {
            elements.push(T::read(read)?);
        }

        Ok(elements)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        (self.len() as SizeType).write(write)?;

        for element in self.iter() {
            element.write(write)?;
        }

        Ok(())
    }
}

