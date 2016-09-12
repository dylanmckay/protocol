use {Type, Error};

use std::collections::HashMap;
use std::io::prelude::*;
use std;

pub type SizeType = u32;

impl<K, V> Type for HashMap<K, V>
    where K: Type + std::hash::Hash + Eq,
          V: Type
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let mut map = HashMap::new();

        let length = SizeType::read(read)?;

        for _ in 0..length {
            let key = K::read(read)?;
            let value = V::read(read)?;

            map.insert(key, value);
        }

        Ok(map)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        (self.len() as SizeType).write(write)?;

        for (key, value) in self.iter() {
            key.write(write)?;
            value.write(write)?;
        }

        Ok(())
    }
}

