use crate::{hint, types, util, Parcel, Error, Settings};
use std::io::prelude::*;
use std;

// The default implementation treats the string as a normal char array.
impl Parcel for std::string::String
{
    const TYPE_NAME: &'static str = "String";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  hints: &mut hint::Hints) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_list(read, settings, hints)?;

        Ok(std::string::String::from_utf8(bytes)?)
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   hints: &mut hint::Hints) -> Result<(), Error> {
        let bytes: Vec<u8> = self.bytes().collect();
        util::write_list(&bytes, write, settings, hints)
    }
}

/// A string with a custom size prefix integer type.
/// `S` - The size prefix type.
#[derive(Clone, Debug, PartialEq)]
pub struct String<S: types::Integer = u32>
{
    pub value: std::string::String,
    _a: std::marker::PhantomData<S>,
}

impl<S: types::Integer> String<S>
{
    pub fn new(s: std::string::String) -> Self {
        String {
            value: s,
            _a: std::marker::PhantomData,
        }
    }
}

impl<S: types::Integer> Parcel for String<S>
{
    const TYPE_NAME: &'static str = "protocol::String<S>";

    fn read_field(read: &mut Read,
            settings: &Settings,
            hints: &mut hint::Hints) -> Result<Self, Error> {
        let bytes = types::Vec::<S, u8>::read_field(read, settings, hints)?;

        Ok(String::new(std::string::String::from_utf8(bytes.elements)?))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   hints: &mut hint::Hints) -> Result<(), Error> {
        let array: types::Vec<S, u8> = types::Vec::new(self.value.bytes().collect());
        array.write_field(write, settings, hints)
    }
}

