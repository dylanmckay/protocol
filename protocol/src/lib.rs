//! Simple packet-based protocol definitions in Rust.
//!
//! * The `Parcel` trait defines any type that can be serialized
//!   to a connection.
//! * The `wire` module deals with transmission of `Parcel`s.
//!
//! # Examples
//!
//! ## Get the raw bytes representing a parcel.
//!
//! ```
//! extern crate protocol;
//! #[macro_use] extern crate protocol_derive;
//!
//! #[derive(Protocol, Debug, PartialEq)]
//! pub struct Health(pub u8);
//!
//! #[derive(Protocol, Debug, PartialEq)]
//! pub struct Player {
//!     pub name: String,
//!     pub health: Health,
//!     pub position: (i16, i16, i16),
//!     pub velocity: (i16, i16, i16),
//! }
//!
//! use protocol::Parcel;
//!
//! assert_eq!(vec![
//!     0, 0, 0, 3, // "Bob" string length prefix
//!     b'B', b'o', b'b', // The string "Bob".
//!     100, // The health byte.
//!     0, 10, 0, 81, 255, 151, // 2-byte x-position, y-position, and z-position.
//!     0, 0, 0, 0, 0, 0, // 2-byte x-velocity, y-velocity, and z-velocity.
//! ], Player {
//!     name: "Bob".to_owned(),
//!     health: Health(100),
//!     position: (10, 81, -105),
//!     velocity: (0, 0, 0),
//! }.raw_bytes(&protocol::Settings::default()).unwrap());
//! ```
//!
//! ## Enums
//!
//! Every enum has a value for each variant used to distinguish
//! between each variant. This can be as simple as a 32-bit integer
//! representing the variant's index. This is called the discriminator.
//!
//! ### String discriminators
//!
//! ```
//! extern crate protocol;
//! #[macro_use] extern crate protocol_derive;
//!
//! // This enum, like all enums, defaults to using String discriminators.
//! #[derive(Protocol, Debug, PartialEq)]
//! enum Foo { A, B, C }
//!
//! // This enum explicitly specifies a string discriminant.
//! //
//! // This is the default anyway and thus it is identical in layout
//! // to the previous enum.
//! #[derive(Protocol, Debug, PartialEq)]
//! #[protocol(discriminant = "string")]
//! enum Bar { A, B, C }
//! ```
//!
//! By default, enums have `String` discriminators. This means that
//! when serializing enum values to and from bytes, unless otherwise
//! specified the variant's full name will be transmitted as a string.
//!
//! This allows variants to be added, modified, or reordered in
//! a backwards-compatible way.
//!
//! ### Integer discriminators
//!
//! ```
//! extern crate protocol;
//! #[macro_use] extern crate protocol_derive;
//!
//! // An enum with integer discriminators.
//! #[derive(Protocol, Debug, PartialEq)]
//! #[protocol(discriminant = "integer")]
//! enum Baz {
//!     Fizz = 0, // override the default first discriminator of 1
//!     Buzz, // defaults to `prior discriminator + 1`, therefore has a discriminator of 1.
//!     Bing, // defaults to `prior discriminator + 1`, therefore has a discriminator of 2.
//!     Boo = 1234,
//!     Beez, // defaults to `prior discriminator + 1`, therefore has a discriminator of 1235.
//! }
//!
//! use protocol::{Enum, Parcel};
//!
//! // By default, integer discriminators are 32-bits.
//! assert_eq!([0u8, 0, 0, 2], &Baz::Bing.discriminator().raw_bytes(&protocol::Settings::default()).unwrap()[..]);
//!
//! assert_eq!(0, Baz::Fizz.discriminator());
//! assert_eq!(1, Baz::Buzz.discriminator());
//! assert_eq!(2, Baz::Bing.discriminator());
//! assert_eq!(1234, Baz::Boo.discriminator());
//! assert_eq!(1235, Baz::Beez.discriminator());
//! ```
//!
//! It is possible to set the underlying integer type via the `#[repr(<type>)]` attribute.
//!
//! ```
//! extern crate protocol;
//! #[macro_use] extern crate protocol_derive;
//!
//! #[derive(Protocol, Debug, PartialEq)]
//! #[protocol(discriminant = "integer")]
//! #[repr(u8)] // Force discriminators to be 8-bit.
//! pub enum Hello {
//!     World(String), SolarSystem(String), LocalGroup(String),
//!     Universe(String), Everything(String),
//! }
//!
//! use protocol::{Enum, Parcel};
//!
//! assert_eq!([2, // 1-byte discriminator
//!             0, 0, 0, 3, // string length
//!             b'f', b'o', b'o', // the string
//!             ], &Hello::SolarSystem("foo".to_owned()).raw_bytes(&protocol::Settings::default()).unwrap()[..]);
//! ```
//!
//! Discriminators can be overriden on a per-variant basis via
//! the `#[protocol(discriminator(<value>))]` attribute.
//!
//! In the case of trivial enums with no fields,
//! the `Variant = <discriminator>` syntax may be used.
//!
//! ```
//! extern crate protocol;
//! #[macro_use] extern crate protocol_derive;
//!
//! #[derive(Protocol, Debug, PartialEq)]
//! #[protocol(discriminant = "integer")]
//! #[repr(u8)] // Force discriminators to be 8-bit.
//! enum Numbered {
//!     A = 1, B = 2, C = 111, D = 67,
//! }
//!
//! #[derive(Protocol, Debug, PartialEq)]
//! #[protocol(discriminant = "integer")]
//! #[repr(u8)] // Force discriminators to be 8-bit.
//! pub enum Hello {
//!     World(String), SolarSystem(String), LocalGroup(String),
//!     #[protocol(discriminator(100))]
//!     Universe(String),
//!     Everything(String),
//! }
//!
//! use protocol::{Enum, Parcel};
//!
//! assert_eq!(111, Numbered::C.discriminator());
//! assert_eq!(67, Numbered::D.discriminator());
//!
//! assert_eq!(1, Hello::World("foo".to_owned()).discriminator());
//! assert_eq!(100, Hello::Universe("foo".to_owned()).discriminator());
//! assert_eq!(101, Hello::Everything("foo".to_owned()).discriminator());
//!
//! ```

pub use self::enum_ty::Enum;
pub use self::parcel::Parcel;
pub use self::errors::{Error, ErrorKind, ResultExt, CharTryFromError, TryFromIntError};
pub use self::settings::*;

mod settings;
#[macro_use]
pub mod types;
#[macro_use]
pub mod wire;

pub mod attributes;
mod enum_ty;
mod errors;
pub mod hint;
mod parcel;
pub mod util;


extern crate byteorder;
extern crate flate2;
#[macro_use]
extern crate error_chain;

#[cfg(feature = "uuid")]
extern crate uuid;
extern crate num_traits;

/// The default byte ordering.
pub type DefaultByteOrder = ::byteorder::BigEndian;

