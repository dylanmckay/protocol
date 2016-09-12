use {Type, Error};

use std::collections::HashSet;
use std::hash::Hash;

use std::io::prelude::*;

impl<T: Type + Eq + Hash> Type for HashSet<T>
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
    pub use std::collections::HashSet;

    describe! serialization {
        it "can be written and then read without changing" {
            let original: HashSet<u32> = [1, 2, 3, 4, 5].iter().cloned().collect();

            let raw_bytes = original.raw_bytes().unwrap();
            let read_deque = HashSet::<u32>::from_raw_bytes(&raw_bytes).unwrap();

            assert_eq!(original, read_deque);
        }
    }
}

