#[cfg(test)]
mod string_discriminants {
    #[allow(unused_imports)]
    use protocol::Parcel;

    #[derive(Protocol, Debug, PartialEq)]
    #[protocol(discriminant = "string")]
    pub enum Axis { X, Y, Z, Other(String), Bimp { val: u64 } }

    fn verify_read_back<P: Parcel + ::std::fmt::Debug + ::std::cmp::PartialEq>(parcel: P) {
        let read_back = P::from_raw_bytes(&parcel.raw_bytes().unwrap()[..]).unwrap();
        assert_eq!(parcel, read_back);
    }

    #[test]
    fn variant_names_are_discriminators() {
        assert_eq!(vec![0, 0, 0, 1, 'X' as _], Axis::X.raw_bytes().unwrap());
        assert_eq!(vec![0, 0, 0, 5, 'O' as _, 't' as _, 'h' as _, 'e' as _, 'r' as _,
                        0, 0, 0, 4, 'r' as _, 'o' as _, 'l' as _, 'l' as _],
                   Axis::Other("roll".to_owned()).raw_bytes().unwrap());
    }

    #[test]
    fn can_write_and_read_back() {
        verify_read_back(Axis::Other("boop".to_owned()));
        verify_read_back(Axis::X);
        verify_read_back(Axis::Y);
        verify_read_back(Axis::Bimp { val: 77 });
    }
}

#[cfg(test)]
mod integer_discriminants {
    #[allow(unused_imports)]
    use protocol::Parcel;

    #[derive(Protocol, Debug, PartialEq, Eq)]
    #[protocol(discriminant = "integer")]
    pub enum BoatKind {
        Speedboat { warp_speed_enabled: bool },
        Dingy(u8, u8),
        Fart,
    }

    #[test]
    fn named_fields_are_correctly_written() {
        assert_eq!(vec![0, 0, 0, 1, 1], BoatKind::Speedboat {
            warp_speed_enabled: true,
        }.raw_bytes().unwrap());
    }

    #[test]
    fn unnamed_fields_are_correctly_written() {
        assert_eq!(vec![0, 0, 0, 2, // discriminator
                        0xf1, 0xed], BoatKind::Dingy(0xf1, 0xed).raw_bytes().unwrap());
    }

    #[test]
    fn unit_variants_are_correctly_written() {
        assert_eq!(vec![0, 0, 0, 3], // discriminator
                   BoatKind::Fart.raw_bytes().unwrap());
    }

    #[test]
    fn named_fields_are_correctly_read() {
        assert_eq!(BoatKind::Speedboat {
            warp_speed_enabled: true,
        }, BoatKind::from_raw_bytes(&[0, 0, 0, 1, 1]).unwrap());
    }

    #[test]
    fn unnamed_fields_are_correctly_read() {
        assert_eq!(BoatKind::Dingy(99, 78),
                   BoatKind::from_raw_bytes(&[0, 0, 0, 2, 99, 78]).unwrap());
    }

    #[test]
    fn unit_variants_are_correctly_read() {
        assert_eq!(BoatKind::Fart,
                   BoatKind::from_raw_bytes(&[0, 0, 0, 3]).unwrap());
    }
}

