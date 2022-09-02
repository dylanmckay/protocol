use crate::{hint, Error, Settings};
use std::io::prelude::*;
use std::io;

/// A value which can be read and written.
///
/// All of the expected standard Rust types implement this.
///
/// Types that implement `Parcel` include (not exhaustive):
///
///   * The Rust primitive types
///     * `u8`, `i8`, `u16`, `i16` ... `u64`
///     * `bool`, `char`
///     * `f32`, `f64`
///     * Tuples
///       * `(T1) where T1: Parcel`
///       * `(T1, T2) where T1: Parcel, T2: Parcel`
///       * `(T1, T2, ... Tn) where T1: Parcel, T2: Parcel, ... Tn: Parcel`
///     * Arrays
///       * `[T; 0] where T: Parcel`
///       * `[T; 1] where T: Parcel`
///       * `[T; 2] where T: Parcel`
///       * `[T; 32] where T: Parcel`
///       * `[T; 40] where T: Parcel`
///       * `[T; 42] where T: Parcel`
///       * ...
///       * `[T; 64] where T: Parcel`
///       * ...
///       * `[T; 1024] where T: Parcel`
///     * `String`
///     * `Option<T: Parcel>`
///     * `Box<T: Parcel>`
///     * `std::ops::Range<T: Parcel>`
///     * Smart pointers
///       * `std::rc::Rc`
///       * `std::sync::Arc`
///     * `std::collections`
///       * `Vec<T: Parcel>`
///       * `VecDeque<T: Parcel>`
///       * `HashMap<T: Parcel>`
///       * `BTreeMap<T: Parcel>`
pub trait Parcel : Sized
{
    /// The textual name of the type.
    const TYPE_NAME: &'static str;

    /// Reads a new item with a fresh set of hints.
    ///
    /// Blocks until a value is received.
    fn read(read: &mut dyn Read,
            settings: &Settings) -> Result<Self, Error> {
        Self::read_field(read, settings, &mut hint::Hints::default())
    }

    /// Reads a value from a stream.
    ///
    /// Parameters:
    ///
    ///   * `hints` - a list of hints accessible by the current
    ///   parcel chain only.
    ///
    /// Blocks until a value is received.
    fn read_field(read: &mut dyn Read,
                  settings: &Settings,
                  hints: &mut hint::Hints) -> Result<Self, Error>;

    /// Writes a value to a stream.
    fn write(&self, write: &mut dyn Write,
             settings: &Settings) -> Result<(), Error> {
        self.write_field(write, settings, &mut hint::Hints::default())
    }

    /// Writes a value to a stream.
    fn write_field(&self, write: &mut dyn Write,
             settings: &Settings,
             hints: &mut hint::Hints) -> Result<(), Error>;

    /// Convers the value into a byte stream that implements `std::io::Read`.
    fn into_stream(self, settings: &Settings)
        -> Result<io::Cursor<Vec<u8>>, Error> {
        self.raw_bytes(settings).map(io::Cursor::new)
    }

    /// Parses a new value from its raw byte representation.
    ///
    /// Returns `Err` if the bytes represent an invalid value.
    fn from_raw_bytes(bytes: &[u8],
                      settings: &Settings) -> Result<Self, Error> {
        let mut hints = hint::Hints::default();
        Self::field_from_raw_bytes(bytes, settings, &mut hints)
    }

    /// Parses a new value from its raw byte representation.
    ///
    /// Returns `Err` if the bytes represent an invalid value.
    fn field_from_raw_bytes(bytes: &[u8],
                            settings: &Settings,
                            hints: &mut hint::Hints) -> Result<Self, Error> {
        let mut buffer = ::std::io::Cursor::new(bytes);
        Self::read_field(&mut buffer, settings, hints)
    }


    /// Gets the raw byte representation of the value.
    fn raw_bytes(&self, settings: &Settings) -> Result<Vec<u8>, Error> {
        self.raw_bytes_field(settings, &mut hint::Hints::default())
    }

    /// Gets the raw bytes of this type as a field of a larger type.
    fn raw_bytes_field(&self,
                       settings: &Settings,
                       hints: &mut hint::Hints) -> Result<Vec<u8>, Error> {
        let mut buffer = io::Cursor::new(Vec::new());
        self.write_field(&mut buffer, settings, hints)?;

        Ok(buffer.into_inner())
    }

    /// Gets the name of the type; `Parcel::TYPE_NAME`.
    fn type_name(&self) -> &'static str { Self::TYPE_NAME }
}

