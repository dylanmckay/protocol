use std::io::prelude::*;

/// Defines serialization settings.
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Settings {
    /// The byte ordering of data transmitted.
    pub byte_order: ByteOrder,
}

/// Specifies the byte order of data transfer.
///
/// # `Eq` implementation
///
/// Here is a list of rules the `Eq` implementation satisfies.
///
///   * `LittleEndian == LittleEndian`
///   * `LittleEndian != BigEndian`
///   * `NativeEndian == LittleEndian` (but only on little endian machines)
///   * `NativeEndian == BigEndian` (but only on big endian machines)
///
/// The `NativeEndian` byte order will successfully match against
/// one of the two real-life byte orders.
#[derive(Copy, Clone, Debug, Eq, PartialOrd, Ord, Hash)]
pub enum ByteOrder {
    /// Least significant byte first.
    LittleEndian,
    /// Most significant byte first.
    BigEndian,
    /// Whatever the byte ordering of the current machine is.
    NativeEndian,
}

#[cfg(target_endian = "little")]
const NATIVE_BYTE_ORDER: ByteOrder = ByteOrder::LittleEndian;
#[cfg(target_endian = "big")]
const NATIVE_BYTE_ORDER: ByteOrder = ByteOrder::BigEndian;

impl ByteOrder {
    /// Resolves the byte order into either little or big endian.
    fn realize(self) -> Self {
        match self {
            ByteOrder::NativeEndian => NATIVE_BYTE_ORDER,
            b => b,
        }
    }
}

impl Default for ByteOrder {
    fn default() -> Self {
        ByteOrder::BigEndian
    }
}

impl ::std::cmp::PartialEq for ByteOrder {
    fn eq(&self, other: &Self) -> bool {
        use ByteOrder::*;

        match (self.realize(), other.realize()) {
            (LittleEndian, LittleEndian) => true,
            (BigEndian, BigEndian) => true,
            _ => false,
        }
    }
}

macro_rules! impl_byte_order_helpers {
    ( $( $ty:ty => [ $read_name:ident : $write_name:ident ] )* ) => {
        impl ByteOrder {
            $(
                pub fn $read_name(&self, read: &mut Read) -> Result<$ty, crate::Error> {
                    use byteorder as bo;

                    Ok(match *self {
                        ByteOrder::LittleEndian => bo::ReadBytesExt::$read_name::<bo::LittleEndian>(read),
                        ByteOrder::BigEndian => bo::ReadBytesExt::$read_name::<bo::BigEndian>(read),
                        ByteOrder::NativeEndian => bo::ReadBytesExt::$read_name::<bo::NativeEndian>(read),
                    }?)
                }

                pub fn $write_name(&self, value: $ty,
                                   write: &mut Write) -> Result<(), crate::Error> {
                    use byteorder as bo;

                    Ok(match *self {
                        ByteOrder::LittleEndian => bo::WriteBytesExt::$write_name::<bo::LittleEndian>(write, value),
                        ByteOrder::BigEndian => bo::WriteBytesExt::$write_name::<bo::BigEndian>(write, value),
                        ByteOrder::NativeEndian => bo::WriteBytesExt::$write_name::<bo::NativeEndian>(write, value),
                    }?)
                }
            )*
        }
    };
}

impl_byte_order_helpers!(
    u16 => [read_u16 : write_u16]
    i16 => [read_i16 : write_i16]
    u32 => [read_u32 : write_u32]
    i32 => [read_i32 : write_i32]
    u64 => [read_u64 : write_u64]
    i64 => [read_i64 : write_i64]
    f32 => [read_f32 : write_f32]
    f64 => [read_f64 : write_f64]
);

