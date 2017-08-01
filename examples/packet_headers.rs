//! An example of packets with common headers.
//! This works because types and packets are the same thing.
//! This means that we can simply have a packet with another packet field.

#[macro_use]
extern crate protocol;

define_packet!(Handshake);

define_packet!(Packet {
    headers: std::collections::HashMap<String, String>,
    kind: PacketKind
});

// Defines a packet kind enum.
define_packet_kind!(PacketKind: u32 {
    0x00 => Handshake
});

fn main() {
    use std::net::TcpStream;

    let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let mut connection = protocol::wire::stream::Connection::new(stream, protocol::wire::middleware::pipeline::default());

    connection.send_packet(&Packet {
        headers: std::collections::HashMap::new(),
        kind: PacketKind::Handshake(Handshake),
    }).unwrap();

    loop {
        if let Some(response) = connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }
}

