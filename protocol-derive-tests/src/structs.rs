#![cfg(test)]

use protocol::Parcel;

#[derive(Protocol, Debug, PartialEq, Eq)]
pub struct Foobar {
    a: u8,
    b: u8,
    c: u8,
}

#[test]
fn named_fields_are_correctly_written() {
    assert_eq!(vec![3, '2' as u8, 1], Foobar {
        a: 3,
        b: '2' as u8,
        c: 1,
    }.raw_bytes().unwrap());
}

#[test]
fn named_fields_are_correctly_read() {
    assert_eq!(Foobar {
        a: 3,
        b: '2' as u8,
        c: 1,
    }, Foobar::from_raw_bytes(&[3, '2' as u8, 1]).unwrap());
}

