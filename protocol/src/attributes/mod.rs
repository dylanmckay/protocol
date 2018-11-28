//! Documentation about the attributes available to `#[derive(Protocol)]`.
//!
//! Here is an example of `#[derive(Protocol)]`.
//!
//! ```
//! #[macro_use] extern crate protocol_derive;
//!
//! #[derive(Protocol)]
//! struct Packet {
//!     version_number: u8,
//!     magic_number: u8,
//!     payload: Vec<u8>,
//! }
//!
//! ```
//!
//! # Attributes that apply to items
//!
//! These attributes apply to structs and enums.
//!
//! ## `#[protocol(length_prefix(<kind>(<length prefix field name>)))]`
//!
//! This attribute allows variable-sized fields to have their sizes
//! specified by an arbitrary integer field in the same struct or enum.
//!
//! Without this attribute, variable-sized fields default to having 32-bit
//! unsigned integer length prefixes prefixed immediately before the field
//! itself.
//!
//! ### Length prefix kinds
//!
//! #### `bytes`
//!
//! ```
//! #[macro_use] extern crate protocol_derive;
//!
//! #[derive(Protocol)]
//! pub struct Foo {
//!     /// This field specifes the length of the last field `reason`.
//!     ///
//!     /// When values of this type are read, the size of `reason` is
//!     /// assumed to be `reason_length` bytes.
//!     pub reason_length: u16,
//!     pub other_stuff_inbetween: [u16; 16],
//!     pub thingy: bool,
//!     /// This field
//!     #[protocol(length_prefix(bytes(reason_length)))]
//!     pub reason: String,
//! }
//! ```
//!
//! This attribute can only be used with named fields. This means structs like
//! `struct Hello(u32)` cannot be supported. This is because the length prefix
//! field must be specified by a name, and therefore only items with named fields
//! can ever have length prefixes.

