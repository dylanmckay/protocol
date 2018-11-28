use protocol::{Parcel, Settings};

#[derive(Protocol, Debug, PartialEq, Eq)]
struct Foo {
    pub reason_length: u16,
    pub other: u64,
    #[protocol(length_prefix(bytes(reason_length)))]
    pub reason: String,
}

