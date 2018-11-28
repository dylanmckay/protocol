use protocol::{Parcel, Settings};

#[derive(Protocol, Debug, PartialEq, Eq)]
struct Foo {
    #[protocol(length_prefix(reason))]
    pub reason_length: u16,
    pub other: u64,
    pub reason: String,
}

