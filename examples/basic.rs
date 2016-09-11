#![feature(question_mark)]

#[macro_use]
extern crate protocol;

define_packet!(Handshake);

define_packet!(Hello {
    id: i64,
    data: Vec<u8>
});

define_packet!(Goodbye {
    id: i64,
    reason: String
});

define_composite_type!(Node {
    name: String,
    enabled: bool
});

// Defines a packet kind enum.
define_packet_kind!(Packet: u32 {
    0x00 => Handshake,
    0x01 => Hello,
    0x02 => Goodbye
});

fn main() {
    use std::net::TcpStream;

    let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let mut connection = protocol::wire::stream::Connection::new(stream);

    connection.send_packet(&Packet::Handshake(Handshake)).unwrap();
    connection.send_packet(&Packet::Hello(Hello { id: 0, data: vec![ 55 ]})).unwrap();
    connection.send_packet(&Packet::Goodbye(Goodbye { id: 0, reason: "leaving".to_string() })).unwrap();

    loop {
        connection.process_incoming_data().unwrap();

        if let Some(response) = connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }
}

