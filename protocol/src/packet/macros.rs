
/// Defines a new struct-based packet.
#[macro_export]
macro_rules! define_packet
{
    // Define a normal packet.
    ( $ty:ident { $( $field_name:ident : $field_ty:ty),+ }) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $ty
        {
            $( pub $field_name : $field_ty ),+
        }

        impl $crate::Parcel for $ty
        {
            const TYPE_NAME: &'static str = stringify!($ty);

            fn read(read: &mut ::std::io::Read,
                    settings: &Settings) -> Result<Self, $crate::Error> {
                #[allow(unused_imports)]
                use $crate::Parcel;

                Ok($ty {
                    $( $field_name : $crate::Parcel::read(read, settings)?, )+
                })
            }

            #[cfg(feature = "tokio")]
            fn read_async(read: &mut ::tokio::io::AsyncRead,
                          settings: &Settings)
                -> Box<::tokio::prelude::Future<Item=Self, Error=$crate::Error> + Send> {
                unimplemented!();
            }

            fn write(&self, write: &mut ::std::io::Write,
                     settings: &Settings) -> Result<(), $crate::Error> {
                #[allow(unused_imports)]
                use $crate::Parcel;

                $( self.$field_name.write(write, settings)?; )+

                Ok(())
            }
        }
    };

    // Define an empty packet.
    ( $ty:ident ) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $ty;

        impl $crate::Parcel for $ty
        {
            const TYPE_NAME: &'static str = stringify!($ty);

            fn read(_read: &mut ::std::io::Read,
                    _: &$crate::Settings) -> Result<Self, $crate::Error> {
                Ok($ty)
            }

            #[cfg(feature = "tokio")]
            fn read_async(read: &mut ::tokio::io::AsyncRead,
                          settings: &Settings)
                -> Box<::tokio::prelude::Future<Item=Self, Error=$crate::Error> + Send> {
                use tokio::prelude::IntoFuture;
                Box::new(Ok($ty).into_future())
            }

            fn write(&self, _write: &mut ::std::io::Write,
                     _: &$crate::Settings) -> Result<(), $crate::Error> {
                Ok(())
            }
        }
    };
}

/// Defines a packet kind enum.
///
/// You can use any type that implements `Parcel` as the packet ID.
#[macro_export]
macro_rules! define_packet_kind
{
    ( $ty:ident : $id_ty:ty { $( $packet_id:expr => $packet_ty:ident ),+ } ) => {
        #[derive(Clone, Debug, PartialEq)]
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

        impl $crate::Parcel for $ty
        {
            const TYPE_NAME: &'static str = stringify!($ty);

            fn read(read: &mut ::std::io::Read,
                    settings: &$crate::Settings) -> Result<Self, $crate::Error> {
                let packet_id = <$id_ty as $crate::Parcel>::read(read, settings)?;

                let packet = match packet_id {
                    $( $packet_id => $ty::$packet_ty(<$packet_ty as $crate::Parcel>::read(read, settings)?), )+
                    _ => return Err($crate::ErrorKind::UnknownPacketId.into()),
                };

                Ok(packet)
            }

            #[cfg(feature = "tokio")]
            fn read_async(read: &mut ::tokio::io::AsyncRead,
                          settings: &$crate::Settings)
                -> Box<::tokio::prelude::Future<Item=Self, Error=$crate::Error> + Send> {
                unimplemented!();
            }

            fn write(&self, write: &mut ::std::io::Write,
                     settings: &$crate::Settings) -> Result<(), $crate::Error> {
                #[allow(unused_imports)]
                use $crate::Parcel;

                self.packet_id().write(write, settings)?;

                match *self {
                    $( $ty::$packet_ty(ref p) => <$packet_ty as $crate::Parcel>::write(p, write, settings)? ),+
                }

                Ok(())
            }
        }
    }
}

