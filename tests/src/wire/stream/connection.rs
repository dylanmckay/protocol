pub use protocol::{Parcel, Settings};
use protocol::wire::stream::Connection;
pub use protocol::wire::middleware;

pub use std::io::Cursor;

#[derive(protocol::Protocol, Clone, Debug, PartialEq, Eq)]
pub struct Ping {
    data: Vec<u8>
}

#[derive(protocol::Protocol, Clone, Debug, PartialEq, Eq)]
#[protocol(discriminant = "integer")]
pub enum PacketKind {
    #[protocol(discriminator(0x00))]
    Ping(Ping),
}

#[test]
fn can_write_and_read_back_data() {
    let settings = Settings::default();
    let ping = PacketKind::Ping(Ping { data: vec![5, 4, 3, 2, 1]});

    let buffer = Cursor::new(Vec::new());
    let mut connection = Connection::new(buffer, middleware::pipeline::default(), settings.clone());

    connection.send_packet(&ping).unwrap();

    // Read the packet back.
    connection.stream.set_position(0);
    let response = connection.receive_packet().unwrap();

    assert_eq!(response.unwrap().raw_bytes(&settings).unwrap(),
               ping.raw_bytes(&settings).unwrap());
}

