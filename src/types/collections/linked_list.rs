use {Type, Error};

use std::collections::LinkedList;

use std::io::prelude::*;

impl<T: Type> Type for LinkedList<T>
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let elements: Vec<T> = Vec::read(read)?;
        Ok(elements.into_iter().collect())
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        let elements: Vec<_> = self.iter().cloned().collect();
        elements.write(write)?;
        Ok(())
    }
}

#[cfg(test)]
mod test
{
    pub use Type;
    pub use std::collections::LinkedList;

    describe! serialization {
        it "can be written and then read without changing" {
            let original: LinkedList<u32> = [1, 2, 3, 4, 5].iter().cloned().collect();

            let raw_bytes = original.raw_bytes().unwrap();
            let read_deque = LinkedList::<u32>::from_raw_bytes(&raw_bytes).unwrap();

            assert_eq!(original, read_deque);
        }
    }
}

