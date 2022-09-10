//! Enable compression.
//! The default middleware pipeline supports compression, but is disabled
//! by default.

use protocol::wire::middleware::{self, compression};
use protocol::wire::stream;

pub const ALGORITHM: compression::Algorithm = compression::Algorithm::Zlib;

#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub struct Hello {
    id: i64,
    data: Vec<u8>
}

fn main() {
    use std::net::TcpStream;

    let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let mut connection = stream::Connection::new(stream, middleware::pipeline::default(), protocol::Settings::default());

    connection.middleware.compression = compression::Compression::Enabled(ALGORITHM);

    connection.send_packet(&Hello { id: 0, data: vec![ 55 ]}).unwrap();

    loop {
        if let Some(response) = connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }
}

