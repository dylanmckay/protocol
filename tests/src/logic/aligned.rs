use protocol::{Parcel, Settings};

/// An example packet with a length prefix disjoint
/// from its data, with the data also
#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
struct Packet {
    /// The length of the 'reason' string.
    pub reason_length: u8,
    /// The version number of the protocol.
    pub version_number: (u32, u32),
    #[protocol(length_prefix(bytes(reason_length)))]
    pub reason: protocol::logic::Aligned<String, u64>,
}

#[test]
fn write_alignment_pads_zero() {
    let raw_bytes = Packet {
        reason_length: 12,
        version_number: (11, 0xdeadbeef),
        reason: "hello world!".to_owned().into(),
    }.raw_bytes(&protocol::Settings::default()).unwrap();
    assert_eq!(&[
        12, // reason length
        0, 0, 0, 11, 0xde, 0xad, 0xbe, 0xef, // version number
        // the string "hello world".
        b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', b'!',
        0x00, 0x00, 0x00, 0x00, // padding bytes to align string to 16 bytes.
        ], &raw_bytes[..]);
}

#[test]
fn read_alignment_pads_zero() {
    let expected_packet = Packet {
        reason_length: 4,
        version_number: (11, 0xdeadbeef),
        reason: "foob".to_owned().into(),
    };

    assert_eq!(expected_packet, Packet::from_raw_bytes(&[
        4, // reason length
        0, 0, 0, 11, 0xde, 0xad, 0xbe, 0xef, // version number
        // the string "foob".
        b'f', b'o', b'o', b'b',
        0x00, 0x00, 0x00, 0x00, // padding bytes to align string to 8 bytes.
    ], &Settings::default()).unwrap());
}

verify_read_back!(length_prefix_with_8_byte_alignment => Packet {
    reason_length: 12,
    version_number: (11, 0xdeadbeef),
    reason: "hello world!".to_owned().into(),
});

