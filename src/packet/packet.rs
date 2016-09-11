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

/// Defines a packet kind enum.
///
/// ```
/// define_packet_kind!(Packet: u32 {
///     0x00 => Handshake,
///     0x01 => Join,
///     0x02 => Kick
/// });
/// ```
///
/// You can use any type that implements `Type` as the packet ID.
macro_rules! define_packet_kind
{
    ( $ty:ident : $id_ty:ty { $( $packet_id:expr => $packet_ty:ident ),+ } ) => {
        #[derive(Clone, Debug)]
        pub enum $ty
        {
            $( $packet_ty($packet_ty) ),+
        }

        impl $ty
        {
            /// Gets the ID of the packet.
            pub fn packet_id(&self) -> $id_ty {
                match *self {
                    $( $ty::$packet_ty(..) => $packet_id ),+
                }
            }
        }

        impl Packet for $ty
        {
            fn read(read: &mut Read) -> Result<Self, Error> {
                let packet_id = <$id_ty as Type>::read(read)?;

                let packet = match packet_id {
                    $( $packet_id => $ty::$packet_ty(<$packet_ty as Packet>::read(read)?), )+
                    _ => return Err(Error::UnknownPacketId),
                };

                Ok(packet)
            }

            fn write(&self, write: &mut Write) -> Result<(), Error> {
                self.packet_id().write(write)?;

                match *self {
                    $( $ty::$packet_ty(ref p) => p.write(write)? ),+
                }

                Ok(())
            }
        }
    }
}

define_packet!(Hello {
    id: u8,
    val: String
});

define_packet_kind!(PacketFoo: u32 {
    0x00 => Hello
});
