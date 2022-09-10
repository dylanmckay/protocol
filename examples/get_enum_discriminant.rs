use protocol::Enum;

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
#[protocol(discriminant = "integer")]
#[repr(u16)]
pub enum PacketKind {
    #[protocol(discriminator(0x00))]
    Handshake(Handshake),
    #[protocol(discriminator(0xaa))]
    Hello(Hello),
    #[protocol(discriminator(0xaf))]
    Goodbye(Goodbye),
}

fn main() {
    println!("enum discriminant 1: {}", PacketKind::Handshake(Handshake).discriminator());
    println!("enum discriminant 2: {}", PacketKind::Goodbye(Goodbye { id: 22, reason: "hello".to_string() }).discriminator());
}
