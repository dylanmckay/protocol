use protocol::{Parcel, Settings};


#[derive(protocol::Protocol, Debug, PartialEq, Eq)]
struct WithLengthPrefixSeparateType {
    pub prefix: Prefix,
    #[protocol(length_prefix(bytes("prefix.reason_length")))]
    pub reason: String,
}

#[derive(protocol::Protocol, Debug, PartialEq, Eq)]
struct Foo<L: Parcel> {
    pub reason_length: u16,
    pub other: u8,
    #[protocol(length_prefix(bytes(reason_length)))]
    pub reason: L,
}

#[derive(protocol::Protocol, Debug, PartialEq, Eq)]
pub struct Prefix {
    pub reason_length: u8,
}

#[derive(protocol::Protocol, Debug, PartialEq, Eq)]
pub struct WithElementsLength {
    pub count: u32,
    pub foo: bool,
    #[protocol(length_prefix(elements(count)))]
    pub data: Vec<u32>,
}

#[test]
fn can_read_length_prefix_5_bytes_string() {
    assert_eq!(Foo {
        reason_length: 5,
        other: 123,
        reason: "hello".to_owned(),
    }, Foo::from_raw_bytes(&[0, 5, 123, b'h', b'e', b'l', b'l', b'o'], &Settings::default()).unwrap());
}

#[test]
fn can_read_length_prefix_8_bytes_u32_array() {
    assert_eq!(Foo {
        reason_length: 8,
        other: 123,
        reason: vec![0x00ff00ff, 0x00ff00ff],
    }, Foo::from_raw_bytes(&[0, 8, 123, 0, !0, 0, !0, 0, !0, 0, !0], &Settings::default()).unwrap());
}


#[test]
fn can_read_length_prefix_3_elements() {
    assert_eq!(WithElementsLength {
        count: 3,
        foo: true,
        data: vec![1, 2, 3],
    }, WithElementsLength::from_raw_bytes(
                           &[0, 0, 0, 3, // disjoint length prefix
                             1, // boolean true
                             0, 0, 0, 1, // 1
                             0, 0, 0, 2, // 2
                             0, 0, 0, 3], // 3
                             &Settings::default()).unwrap());
}

