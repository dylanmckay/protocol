use {Type, Error};

use std::io::prelude::*;
use std::fmt;

/// A packet enum.
pub trait PacketKind : Clone + fmt::Debug
{
    fn read(read: &mut Read) -> Result<Self, Error>;
    fn write(&self, write: &mut Write) -> Result<(), Error>;
}

/// A specific packet type.
pub trait Packet : Clone + fmt::Debug
{
    fn read(read: &mut Read) -> Result<Self, Error>;
    fn write(&self, write: &mut Write) -> Result<(), Error>;
}

macro_rules! define_packet
{
    ( $ty:ident { $( $field_name:ident : $field_ty:ty),+ }) => {
        #[derive(Clone, Debug)]
        pub struct $ty
        {
            $( pub $field_name : $field_ty ),+
        }

        impl Packet for $ty
        {
            fn read(read: &mut Read) -> Result<Self, Error> {
                Ok($ty {
                    $( $field_name : <$field_ty as Type>::read(read)?, )+
                })
            }

            fn write(&self, write: &mut Write) -> Result<(), Error> {
                $( self.$field_name.write(write)?; )+

                Ok(())
            }
        }
    }
}

define_packet!(Hello {
    id: u8,
    val: String
});

