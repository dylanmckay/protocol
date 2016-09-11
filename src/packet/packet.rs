use Error;

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

#[macro_export]
macro_rules! define_packet
{
    // Define a normal packet.
    ( $ty:ident { $( $field_name:ident : $field_ty:ty),+ }) => {
        #[derive(Clone, Debug)]
        pub struct $ty
        {
            $( pub $field_name : $field_ty ),+
        }

        impl $crate::Packet for $ty
        {
            fn read(read: &mut ::std::io::Read) -> Result<Self, $crate::Error> {
                #[allow(unused_imports)]
                use $crate::Type;

                Ok($ty {
                    $( $field_name : <$field_ty as $crate::Type>::read(read)?, )+
                })
            }

            fn write(&self, write: &mut ::std::io::Write) -> Result<(), $crate::Error> {
                #[allow(unused_imports)]
                use $crate::Type;

                $( self.$field_name.write(write)?; )+

                Ok(())
            }
        }
    };

    // Define an empty packet.
    ( $ty:ident ) => {
        #[derive(Clone, Debug)]
        pub struct $ty;

        impl $crate::Packet for $ty
        {
            fn read(_read: &mut ::std::io::Read) -> Result<Self, $crate::Error> {
                Ok($ty)
            }

            fn write(&self, _write: &mut ::std::io::Write) -> Result<(), $crate::Error> {
                Ok(())
            }
        }
    };
}

/// Defines a packet kind enum.
///
/// You can use any type that implements `Type` as the packet ID.
#[macro_export]
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

        impl $crate::PacketKind for $ty
        {
            fn read(read: &mut ::std::io::Read) -> Result<Self, $crate::Error> {
                let packet_id = <$id_ty as $crate::Type>::read(read)?;

                let packet = match packet_id {
                    $( $packet_id => $ty::$packet_ty(<$packet_ty as $crate::Packet>::read(read)?), )+
                    _ => return Err($crate::Error::UnknownPacketId),
                };

                Ok(packet)
            }

            fn write(&self, write: &mut ::std::io::Write) -> Result<(), $crate::Error> {
                use $crate::Type;

                self.packet_id().write(write)?;

                match *self {
                    $( $ty::$packet_ty(ref p) => <$packet_ty as $crate::Packet>::write(p, write)? ),+
                }

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    define_packet!(Handshake);
    define_packet!(Kick);

    define_packet!(Hello {
        id: i64,
        data: Vec<u8>
    });

    define_packet!(Goodbye {
        id: i64,
        reason: String
    });

    define_packet!(Properties {
        properties: ::std::collections::HashMap<String, bool>
    });

    define_packet_kind!(Packet: u32 {
        0x00 => Handshake,
        0x01 => Kick,
        0x02 => Hello,
        0x03 => Goodbye,
        0x04 => Properties
    });

    describe! packets {
        before_each {
            let hello = Hello { id: 5678, data: vec![1, 2, 3] };
            let goodbye = Goodbye { id: 8765, reason: "um".to_string() };
        }

        describe! packet_ids {
            describe! numerical {
                it "gets the corrrect ids" {
                    assert_eq!(Packet::Handshake(Handshake).packet_id(), 0x00);
                    assert_eq!(Packet::Kick(Kick).packet_id(), 0x01);

                    assert_eq!(Packet::Hello(hello).packet_id(), 0x02);
                    assert_eq!(Packet::Goodbye(goodbye).packet_id(), 0x03);
                }
            }
        }
    }
}

