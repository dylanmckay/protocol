use {Parcel, Error, Settings};
use {hint, util};

use std::io::prelude::*;
use std::{marker, mem};

/// A parcel that wraps another parcel with a size alignment requirement.
///
/// Null bytes are inserted when writing to ensure the type is padded
/// fit the alignment requirements.
///
/// Padding bytes are ignored when reading.
///
/// Type parameters:
///
///   * The alignment is always rounded up to a multiple of `ToSizeOf`.
///     * The poor man's constant generics
pub struct Aligned<ToSizeOf, T> {
    /// The inner value.
    inner: T,
    _phantom: marker::PhantomData<ToSizeOf>
}

impl<ToSizeOf, T> Aligned<ToSizeOf, T>
    where T: Parcel
{
    /// Gets the alignment in bytes of this type.
    ///
    /// Serialized values must have lengths in multiples
    /// of this alignment.
    pub fn alignment_bytes() -> usize {
        mem::size_of::<ToSizeOf>()
    }
}

impl<ToSizeOf, T> Parcel for Aligned<ToSizeOf, T>
    where T: Parcel
{
    const TYPE_NAME: &'static str = "Aligned<T>";

    fn read(read: &mut Read,
            settings: &mut Settings,
            hints: &mut hint::Hints) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write(&self, write: &mut Write,
             settings: &Settings) -> Result<(), Error> {
        let unaligned_bytes = self.inner.raw_bytes(settings)?;
        let aligned_bytes = util::align_bytes(Self::alignment_bytes(), unaligned_bytes);

        // Write aligned bytes to the stream.
        assert!(aligned_bytes.len() % Self::alignment_bytes() == 0,
                "aligned bytes are not actually aligned");
        write.write(&aligned_bytes)?;
        Ok(())
    }
}

