use {Parcel, Error, Settings};
use hint;

use std::collections::{HashMap, BTreeMap};
use std::hash::Hash;

use std::io::prelude::*;

pub type SizeType = u32;

macro_rules! impl_map_type {
    ( $ty:ident => K: $( $k_pred:ident ),+ ) => {
        impl<K, V> Parcel for $ty<K, V>
            where K: Parcel + $( $k_pred +)+,
                  V: Parcel
        {
            const TYPE_NAME: &'static str = stringify!($ty<K,V>);

            fn read_field(read: &mut Read,
                          settings: &Settings,
                          _: &mut hint::Hints) -> Result<Self, Error> {
                let mut map = $ty::new();

                let length = SizeType::read(read, settings)?;

                for _ in 0..length {
                    let key = K::read(read, settings)?;
                    let value = V::read(read, settings)?;

                    map.insert(key, value);
                }

                Ok(map)
            }

            fn write_field(&self, write: &mut Write,
                           settings: &Settings,
                           _: &mut hint::Hints) -> Result<(), Error> {
                (self.len() as SizeType).write(write, settings)?;

                for (key, value) in self.iter() {
                    key.write(write, settings)?;
                    value.write(write, settings)?;
                }

                Ok(())
            }
        }
    }
}

impl_map_type!(HashMap => K: Hash, Eq);
impl_map_type!(BTreeMap => K: Ord);

