//! Helper functions for dealing with sets or lists of parcels.

use {Parcel, Error, TryFromIntError, Settings};
use types::Integer;

use std::io::prelude::*;

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

/// Reads a length-prefixed list from a stream.
pub fn read_list_ext<S,T>(read: &mut Read,
                          settings: &Settings)
    -> Result<Vec<T>, Error>
    where S: Integer,
          T: Parcel {
    let size = S::read(read, settings)?;
    let size: usize = size.to_usize().ok_or(TryFromIntError{ })?;

    read_items(size, read, settings).map(|i| i.collect())
}

/// Writes a length-prefixed list to a stream.
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

    write_items(write, elements.into_iter(), settings)?;

    Ok(())
}

/// Align input bytes so the total size if a multiple of
/// the specified alignment.
pub fn align_bytes(align_to: usize,
                   bytes: Vec<u8>)
    -> Vec<u8> {
    // Thanks for the formula Ned!
    // https://stackoverflow.com/a/11642218
    let extra_padding_needed = (align_to - (bytes.len() % align_to)) % align_to;

    let extra_padding = (0..).into_iter().take(extra_padding_needed).map(|_| 0x00);

    let bytes: Vec<_> = bytes.into_iter().chain(extra_padding).collect();
    assert_eq!(0, bytes.len() % align_to,
            "failed to align");
    bytes
}

#[cfg(test)]
mod test {
    mod alignment {
        use super::super::*;

        #[test]
        fn test_aligning_when_none_needed() {
            assert_eq!(vec![1, 2], align_bytes(1, vec![1, 2]));
            assert_eq!(vec![1, 2], align_bytes(2, vec![1, 2]));
        }

        #[test]
        fn test_align_to_3_with_size_2() {
            assert_eq!(vec![1, 2, 0], align_bytes(3, vec![1, 2]));
        }

        #[test]
        fn test_align_to_4_with_size_2() {
            assert_eq!(vec![1, 2, 0, 0], align_bytes(4, vec![1, 2]));
        }

        #[test]
        fn test_align_to_3_with_size_5() {
            assert_eq!(vec![1, 2, 3, 4, 5, 0], align_bytes(3, vec![1, 2, 3, 4, 5]));
        }

        #[test]
        fn test_align_to_4_with_size_97() {
            let original = [1; 97];
            let aligned = align_bytes(4, original.to_vec());

            let count_ones = aligned.iter().filter(|&&i| i == 1).count();
            let count_zeros = aligned.iter().filter(|&&i| i == 0).count();

            assert_eq!(97, count_ones);
            assert_eq!(3, count_zeros);
        }
    }
}

