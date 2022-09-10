#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub struct Handshake;

#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub struct Hello {
    id: i64,
    data: Vec<u8>,
}

#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub struct Goodbye {
    id: i64,
    reason: String,
}

#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub struct Node {
    name: String,
    enabled: bool
}

// Defines a packet kind enum.
#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
pub enum Packet {
    #[protocol(discriminator(0x00))]
    Handshake(Handshake),
    #[protocol(discriminator(0x01))]
    Hello(Hello),
    #[protocol(discriminator(0x02))]
    Goodbye(Goodbye),
}

fn main() {
    use std::net::TcpStream;

    let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let settings = protocol::Settings {
        byte_order: protocol::ByteOrder::LittleEndian,
        ..Default::default()
    };
    let mut connection = protocol::wire::stream::Connection::new(stream, protocol::wire::middleware::pipeline::default(), settings);

    connection.send_packet(&Packet::Handshake(Handshake)).unwrap();
    connection.send_packet(&Packet::Hello(Hello { id: 0, data: vec![ 55 ]})).unwrap();
    connection.send_packet(&Packet::Goodbye(Goodbye { id: 0, reason: "leaving".to_string() })).unwrap();

    loop {
        if let Some(response) = connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }
}

