#[allow(unused_imports)]
use protocol::Parcel;

#[derive(Protocol, Debug, PartialEq, Eq)]
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

