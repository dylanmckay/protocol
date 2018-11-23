use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Settings {
    /// The byte ordering of data transmitted.
    pub byte_order: ByteOrder,
}

/// Specifies the byte order of data transfer.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
    NativeEndian,
}

impl Default for ByteOrder {
    fn default() -> Self {
        ByteOrder::BigEndian
    }
}

macro_rules! impl_byte_order_helpers {
    ( $( $ty:ty => [ $read_name:ident : $write_name:ident ] )* ) => {
        impl ByteOrder {
            $(
                pub fn $read_name(&self, read: &mut Read) -> Result<$ty, ::Error> {
                    use byteorder as bo;

                    Ok(match *self {
                        ByteOrder::LittleEndian => bo::ReadBytesExt::$read_name::<bo::LittleEndian>(read),
                        ByteOrder::BigEndian => bo::ReadBytesExt::$read_name::<bo::BigEndian>(read),
                        ByteOrder::NativeEndian => bo::ReadBytesExt::$read_name::<bo::NativeEndian>(read),
                    }?)
                }

                pub fn $write_name(&self, value: $ty,
                                   write: &mut Write) -> Result<(), ::Error> {
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

