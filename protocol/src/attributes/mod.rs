//! Documentation about the attributes available to `#[derive(protocol::Protocol)]`.
//!
//! Here is an example of `#[derive(Protocol)]`.
//!
//! ```
//! #[derive(protocol::Protocol)]
//! struct Packet {
//!     version_number: u8,
//!     magic_number: u8,
//!     payload: Vec<u8>,
//! }
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
//! When the length prefix type is `bytes`, the length prefix
//! represents the total number of bytes that make up a field.
//!
//! ```
//! #[derive(protocol::Protocol)]
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
//! #### `elements`
//!
//! When the length prefix type is 'elements', the length prefix
//! represents the number of elements in a collection or list.
//!
//! ```
//! #[derive(protocol::Protocol)]
//! pub struct Bar {
//!     /// This field specifes the number of elements in 'data'.
//!     pub reason_length: u16,
//!     pub other_stuff_inbetween: [u16; 16],
//!     pub thingy: bool,
//!     /// This field
//!     #[protocol(length_prefix(elements(reason_length)))]
//!     pub reason: Vec<(u32, u32)>,
//! }
//! ```
//!
//! # Notes
//!
//! This attribute can only be used with named fields. This means structs like
//! `struct Hello(u32)` cannot be supported. This is because the length prefix
//! field must be specified by a name, and therefore only items with named fields
//! can ever have length prefixes.
//!
//! ## Length prefixes placed different structs
//!
//! It is possible for a field one one struct to specify the length of a field
//! in another struct, so long as both structs are fields within a parent struct
//! and the struct defining the length appears earlier than the one whose length
//! is being described.
//!
//! In this case, the length prefix field must be double quoted.
//!
//! `#[protocol(length_prefix(bytes("sibling_field.nested_field.value")))]`
//!
//! Example:
//!
//! ```
//! #[derive(protocol::Protocol)]
//! struct Packet {
//!     /// The length of the adjacent 'reason' field is nested under this field.
//!     pub packet_header: PacketHeader,
//!     /// The length of this field is specified by the packet header.
//!     #[protocol(length_prefix(bytes("packet_header.reason_length")))]
//!     pub reason: String,
//! }
//!
//! #[derive(protocol::Protocol)]
//! pub struct PacketHeader {
//!     pub reason_length: u16,
//! }
//! ```

