use {Parcel, Error, TryFromIntError, Settings};
use types::Integer;

use std::io::prelude::*;

/// The integer type that we will use to send length prefixes.
pub type SizeType = u32;

/// Reads a length-prefixed list from a stream.
pub fn read_list<T>(read: &mut Read,
                    settings: &Settings)
    -> Result<Vec<T>, Error>
    where T: Parcel {
    self::read_list_ext::<SizeType, T>(read, settings)
}

/// Writes a length-prefixed list to a stream.
pub fn write_list<'a,T,I>(write: &mut Write,
                          elements: I,
                          settings: &Settings)
    -> Result<(), Error>
    where T: Parcel+'a,
          I: IntoIterator<Item=&'a T> {
    self::write_list_ext::<SizeType, T, I>(write, elements, settings)
}

pub fn read_list_ext<S,T>(read: &mut Read,
                          settings: &Settings)
    -> Result<Vec<T>, Error>
    where S: Integer,
          T: Parcel {
    let size = S::read(read, settings)?;
    let size: usize = size.to_usize().ok_or(TryFromIntError{ })?;
    let mut elements = Vec::with_capacity(size);

    for _ in 0..size {
        let element = T::read(read, settings)?;
        elements.push(element);
    }
    Ok(elements)
}

pub fn write_list_ext<'a,S,T,I>(write: &mut Write,
                                elements: I,
                                settings: &Settings)
    -> Result<(), Error>
    where S: Integer,
          T: Parcel+'a,
          I: IntoIterator<Item=&'a T> {
    let elements: Vec<_> = elements.into_iter().collect();
    let length = S::from_usize(elements.len()).ok_or(TryFromIntError{ })?;
    length.write(write, settings)?;

    for element in elements.into_iter() {
        element.write(write, settings)?;
    }

    Ok(())
}
