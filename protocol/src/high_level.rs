use crate::{hint, Error, Parcel, Settings};
use std::io::prelude::*;
use std::fmt;

/// A high level abstraction over a lower-level `Parcel`.
///
/// Automatically marshals values to and from the high level
/// and low level types for serialization.
///
/// # Example
///
/// ```
/// #[macro_use] extern crate protocol_derive;
/// extern crate protocol;
///
/// use protocol::Parcel;
///
/// #[derive(Protocol)]
/// pub struct RawPacket {
///     opcode: u8,
///     magic_number: u8,
///     payload: Vec<u8>,
///     version: (u8, u8),
/// }
///
/// #[derive(Debug, Clone)]
/// pub enum Login {
///     Success {
///         message: String,
///     },
///     Failure {
///         code: FailureCode,
///         response: String,
///     },
/// }
///
/// impl protocol::HighLevel for Login {
///     type LowLevel = RawPacket;
///
///     fn into_low_level(self) -> RawPacket {
///         match self {
///             Login::Success { message } => {
///                 RawPacket {
///                     opcode: 0,
///                     magic_number: 0xaa,
///                     payload: message.into_bytes(),
///                     version: (11, 0),
///                 }
///             },
///             Login::Failure { .. } => unimplemented!(),
///         }
///     }
///
///     fn from_low_level(low_level: RawPacket,
///                       _: &mut std::io::Read,
///                       _: &protocol::Settings,
///                       _: &mut protocol::hint::Hints)
///         -> Result<Self, protocol::Error> {
///         match low_level.opcode {
///             0 => Ok(Login::Success { message: String::from_utf8(low_level.payload).unwrap() }),
///             1 => Ok(Login::Failure {
///                 code: FailureCode::MyDogAteMyHomework,
///                 response: String::from_utf8(low_level.payload[1..].to_owned()).unwrap() }),
///             _ => unreachable!(),
///         }
///     }
/// }
///
/// #[derive(Debug, Clone)]
/// pub enum FailureCode {
///     MyDogAteMyHomework,
/// }
///
/// let high_level = Login::Success { message: "Hi!".to_owned() };
/// assert_eq!(11, high_level.raw_bytes(&protocol::Settings::default()).unwrap().len());
/// ```
pub trait HighLevel : Clone + fmt::Debug {
    /// The lower-level representation of this type.
    type LowLevel: Parcel;

    /// Converts this high level type into its lower-level representation.
    fn into_low_level(self) -> Self::LowLevel;

    /// Creates a high-level abstraction over a lower-level value.
    ///
    /// The method has access to the reader post-parsing of the low level type.
    /// It is not necessary to use this if not needed.
    fn from_low_level(value: Self::LowLevel,
                      subsequent_reader: &mut dyn Read,
                      settings: &Settings,
                      hints: &mut hint::Hints) -> Result<Self, Error>;
}

impl<H> Parcel for H
    where H: HighLevel {
    const TYPE_NAME: &'static str = "HighLevel";

    fn read_field(read: &mut dyn Read,
                  settings: &Settings,
                  hints: &mut hint::Hints) -> Result<Self, Error> {
        let low_level = H::LowLevel::read_field(read, settings, hints)?;
        H::from_low_level(low_level, read, settings, hints)
    }

    fn write_field(&self,
                   write: &mut dyn Write,
                   settings: &Settings,
                   hints: &mut hint::Hints) -> Result<(), Error> {
        let low_level = self.clone().into_low_level();
        low_level.write_field(write, settings, hints)
    }
}

