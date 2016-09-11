# protocol

[![Build Status](https://travis-ci.org/dylanmckay/protocol.svg?branch=master)](https://travis-ci.org/dylanmckay/protocol)
[![Crates.io](https://img.shields.io/crates/v/protocol.svg)](https://crates.io/crates/protocol)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Easy protocol definitions in Rust.

Requires the nightly compiler.

## Example

```rust
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

// Defines a packet kind enum.
define_packet_kind!(Packet: u32 {
    0x00 => Handshake,
    0x02 => Hello,
    0x03 => Goodbye,
});

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();

    let mut transport = protocol::transport::Simple::new();

    transport.send_packet(&mut stream, &Handshake).unwrap();
    transport.send_packet(&mut stream, &Hello { id: 0, data: vec![ 55 ]}).unwrap();
    transport.send_packet(&mut stream, &Goodbye { id: 0, reason: "leaving".to_string() }).unwrap();
}
```

