use {Error, Settings};
use std::io::prelude::*;
use std::io;

#[cfg(feature = "tokio")]
use tokio::prelude::*;

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
///     * Smart pointers
///       * `std::rc::Rc`
///       * `std::sync::Arc`
///     * `std::collections`
///       * `Vec<T: Parcel>`
///       * `VecDeque<T: Parcel>`
///       * `HashMap<T: Parcel>`
///       * `BTreeMap<T: Parcel>`
pub trait Parcel : Sized + Send
{
    /// The textual name of the type.
    const TYPE_NAME: &'static str;

    /// Reads a value from a stream.
    ///
    /// Blocks until a value is received.
    fn read(read: &mut Read,
            settings: &Settings) -> Result<Self, ::Error>;

    /// Writes a value to a stream.
    fn write(&self, write: &mut Write,
             settings: &Settings) -> Result<(), ::Error>;

    /// Parses a new value from its raw byte representation.
    ///
    /// Returns `Err` if the bytes represent an invalid value.
    fn from_raw_bytes(bytes: &[u8],
                      settings: &Settings) -> Result<Self, ::Error> {
        let mut buffer = ::std::io::Cursor::new(bytes);
        Self::read(&mut buffer, settings)
    }

    /// Creates a future which asynchronously reads a value from the stream.
    ///
    /// On success, the future yields a tuple containing the remaining stream
    /// and the parsed parcel.
    #[cfg(feature = "tokio")]
    fn read_async<R>(read: R,
                     settings: &Settings)
        -> Box<Future<Item=(R, Self), Error=Error> + Send>
        where R: AsyncRead;

    /// Gets the raw byte representation of the value.
    fn raw_bytes(&self, settings: &Settings) -> Result<Vec<u8>, ::Error> {
        let mut buffer = io::Cursor::new(Vec::new());
        self.write(&mut buffer, settings)?;

        Ok(buffer.into_inner())
    }

    /// Gets the name of the type; `Parcel::TYPE_NAME`.
    fn type_name(&self) -> &'static str { Self::TYPE_NAME }
}

