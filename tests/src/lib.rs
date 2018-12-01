#![cfg(test)]

extern crate protocol;
#[macro_use] extern crate protocol_derive;

macro_rules! verify_read_back {
    ($name:ident => $parcel:expr) => {
        pub mod $name {
            use protocol::{self, Parcel, Settings};
            use super::*;

            fn verify_read_back(settings: &Settings) {
                let read_back = Parcel::from_raw_bytes(&$parcel.raw_bytes(&settings).unwrap()[..], &settings).unwrap();
                assert_eq!($parcel, read_back);
            }

            #[test]
            fn can_read_back_default_settings() {
                verify_read_back(&protocol::Settings::default());
            }

            mod byte_order {
                use super::*;
                use protocol::{ByteOrder, Settings};

                #[test]
                fn can_read_back_in_big_endian() {
                    verify_read_back(&Settings {
                        byte_order: ByteOrder::BigEndian,
                        ..Settings::default()
                    });
                }

                #[test]
                fn can_read_back_in_little_endian() {
                    verify_read_back(&Settings {
                        byte_order: ByteOrder::LittleEndian,
                        ..Settings::default()
                    });
                }

                #[test]
                fn can_read_back_in_native_endian() {
                    verify_read_back(&Settings {
                        byte_order: ByteOrder::NativeEndian,
                        ..Settings::default()
                    });
                }
            }
        }
    };
}

#[cfg(test)] mod enums;
#[cfg(test)] mod enum_trait;
#[cfg(test)] mod hints;
#[cfg(test)] mod length_prefix;
#[cfg(test)] mod structs;
