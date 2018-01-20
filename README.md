# protocol

[![Build Status](https://travis-ci.org/dylanmckay/protocol.svg?branch=master)](https://travis-ci.org/dylanmckay/protocol)
[![Crates.io](https://img.shields.io/crates/v/protocol.svg)](https://crates.io/crates/protocol)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

[Documentation](https://docs.rs/protocol)

Easy protocol definitions in Rust.

This crate adds a custom derive that can be added to types, allowing
structured data to be sent and received from any IO stream.

## Under the hood

The most interesting part here is the [`protocol::Parcel`](https://docs.rs/protocol/0.3.3/protocol/trait.Parcel.html) trait. Any type that implements this trait can then be serialised to and from a byte stream. All primitive types, standard collections, tuples, and arrays implement this trait.

This crate also provides:

* [TCP](https://docs.rs/protocol/0.3.3/protocol/wire/stream/index.html) and [UDP](https://docs.rs/protocol/0.3.3/protocol/wire/dgram/index.html) modules for easy sending and receicing of `Parcel`s
* A generic [middleware](https://docs.rs/protocol/0.3.3/protocol/wire/middleware/index.html) library for automatic transformation of sent/received data
  * Middleware has already been written to support [compression](https://docs.rs/protocol/0.3.3/protocol/wire/middleware/compression/index.html)
  * Custom middleware can be implemented via a trait with two methods

Checkout the [examples](./examples) folder for usage.


## Example

```rust
#[macro_use] extern crate protocol_derive;
#[macro_use] extern crate protocol;

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Handshake;

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Hello {
    id: i64,
    data: Vec<u8>,
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Goodbye {
    id: i64,
    reason: String,
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Node {
    name: String,
    enabled: bool
}

// Defines a packet kind enum.
define_packet_kind!(Packet: u32 {
    0x00 => Handshake,
    0x01 => Hello,
    0x02 => Goodbye
});

fn main() {
    use std::net::TcpStream;

    let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let mut connection = protocol::wire::stream::Connection::new(stream, protocol::wire::middleware::pipeline::default());

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

```

