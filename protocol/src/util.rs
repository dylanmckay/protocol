//! Helper functions for dealing with sets or lists of parcels.

use {Parcel, Error, ErrorKind, TryFromIntError, Settings};
use hint;
use types::Integer;

use std::io::prelude::*;
use std::io;

/// The integer type that we will use to send length prefixes.
pub type SizeType = u32;

/// Reads a string of specified length from a stream.
pub fn read_string(byte_count: usize,
                   read: &mut Read,
                   settings: &Settings)
    -> Result<String, Error> {
    let bytes: Vec<u8> = read_items(byte_count, read, settings)?.collect();
    String::from_utf8(bytes).map_err(Into::into)
}

/// Reads a specified number of items from a stream.
pub fn read_items<T>(item_count: usize,
                     read: &mut Read,
                     settings: &Settings)
    -> Result<impl Iterator<Item=T>, Error>
    where T: Parcel {
    let mut elements = Vec::with_capacity(item_count);

    for _ in 0..item_count {
        let element = T::read(read, settings)?;
        elements.push(element);
    }
    Ok(elements.into_iter())
}

/// Writes an iterator of parcels to the stream.
///
/// Does not include a length prefix.
pub fn write_items<'a,T>(write: &mut Write,
                         items: impl IntoIterator<Item=&'a T>,
                         settings: &Settings)
    -> Result<(), Error>
    where T: Parcel + 'a {
    for item in items.into_iter() {
        item.write(write, settings)?;
    }
    Ok(())

}

/// Reads a length-prefixed list from a stream.
pub fn read_list<T>(read: &mut Read,
                    settings: &Settings,
                    hints: &mut hint::Hints)
    -> Result<Vec<T>, Error>
    where T: Parcel {
    self::read_list_ext::<SizeType, T>(read, settings, hints)
}

/// Writes a length-prefixed list to a stream.
pub fn write_list<'a,T,I>(write: &mut Write,
                          elements: I,
                          settings: &Settings,
                          hints: &mut hint::Hints)
    -> Result<(), Error>
    where T: Parcel+'a,
          I: IntoIterator<Item=&'a T> {
    self::write_list_ext::<SizeType, T, I>(write, elements, settings, hints)
}

/// Reads a length-prefixed list from a stream.
pub fn read_list_ext<S,T>(read: &mut Read,
                          settings: &Settings,
                          hints: &mut hint::Hints)
    -> Result<Vec<T>, Error>
    where S: Integer,
          T: Parcel {
    match hints.current_field_length() {
        Some(length) => {
            match length.kind {
                hint::LengthPrefixKind::Bytes => {
                    let byte_count = length.length;

                    // First, read all bytes of the list without processing them.
                    let bytes: Vec<u8> = read_items(byte_count, read, settings)?.collect();
                    let mut read_back_bytes = io::Cursor::new(bytes);

                    // Then, parse the items until we reach the end of the buffer stream.
                    let mut items = Vec::new();
                    // FIXME: potential DoS vector, should timeout.
                    while read_back_bytes.position() < byte_count as u64 {
                        let item = match T::read(&mut read_back_bytes, settings).map_err(|e| e.0) {
                            Ok(item) => item,
                            Err(ErrorKind::Io(ref io)) if io.kind() == io::ErrorKind::UnexpectedEof => {
                                // FIXME: make this a client error.
                                panic!("length prefix in bytes does not match actual size");
                            },
                            Err(e) => return Err(e.into()),
                        };
                        items.push(item);
                    }

                    Ok(items)
                },
            }
        },
        None => {
            // We do not know the length in the field in advance, therefore there
            // the length prefix is not disjoint.
            let size = S::read(read, settings)?;
            let size: usize = size.to_usize().ok_or(TryFromIntError{ })?;

            read_items(size, read, settings).map(|i| i.collect())
        },
    }
}

/// Writes a length-prefixed list to a stream.
pub fn write_list_ext<'a,S,T,I>(write: &mut Write,
                                elements: I,
                                settings: &Settings,
                                hints: &mut hint::Hints)
    -> Result<(), Error>
    where S: Integer,
          T: Parcel+'a,
          I: IntoIterator<Item=&'a T> {
    let elements: Vec<_> = elements.into_iter().collect();

    match hints.current_field_length() {
        // If there is an existing length prefix, don't bother sending another.
        Some(_length) => {
            ()
        },
        // The length is not known, send a prefix.
        _ => {
            let length = S::from_usize(elements.len()).ok_or(TryFromIntError{ })?;
            length.write(write, settings)?;

        },
    }
    write_items(write, elements.into_iter(), settings)?;

    Ok(())
}

