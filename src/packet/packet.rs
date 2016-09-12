use Error;

use std::io::prelude::*;
use std::io::Cursor;
use std::fmt;

/// A packet enum.
pub trait PacketKind : Clone + fmt::Debug
{
    fn read(read: &mut Read) -> Result<Self, Error>;
    fn write(&self, write: &mut Write) -> Result<(), Error>;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut buffer = Cursor::new(bytes);
        Self::read(&mut buffer)
    }

    fn bytes(&self) -> Result<Vec<u8>, Error> {
        let mut buffer = Cursor::new(Vec::new());
        self.write(&mut buffer)?;
        Ok(buffer.into_inner())
    }
}

/// A specific packet type.
pub trait Packet : Clone + fmt::Debug
{
    fn read(read: &mut Read) -> Result<Self, Error>;
    fn write(&self, write: &mut Write) -> Result<(), Error>;
}

#[cfg(test)]
#[allow(unused_variables)]
mod test {
    pub use PacketKind;

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
            let hello = Hello { id: 55, data: vec![1, 2, 3] };
            let goodbye = Goodbye { id: 8765, reason: "um".to_string() };

            let hello_expected_bytes = &[
                0x00, 0x00, 0x00, 0x02, // Packet ID
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 55, // 'id' field
                0x00, 0x00, 0x00, 0x03, // 'data' length
                0x01, 0x02, 0x03, // 'data' array
            ];
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

            describe! writing {
                it "writes the correct values" {
                    let packet = Packet::Hello(hello);

                    assert_eq!(&packet.bytes().unwrap(), hello_expected_bytes);
                }
            }

            describe! reading {
                it "reads the correct values" {
                    let packet = Packet::from_bytes(hello_expected_bytes).unwrap();

                    assert_eq!(packet.bytes().unwrap(), hello_expected_bytes);
                }
            }
        }
    }
}

