use {primitives, Parcel, Error, DynArray};
use std::io::prelude::*;
use std;

// The default implementation treats the string as a normal char array.
impl Parcel for std::string::String
{
    const TYPE_NAME: &'static str = "String";

    fn read(read: &mut Read) -> Result<Self, Error> {
        let bytes = Vec::<u8>::read(read)?;

        Ok(std::string::String::from_utf8(bytes)?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        let bytes: Vec<u8> = self.bytes().collect();
        bytes.write(write)
    }
}

/// A string with a custom size prefix integer type.
/// `S` - The size prefix type.
#[derive(Clone, Debug, PartialEq)]
pub struct String<S: primitives::Integer = u32>
{
    pub value: std::string::String,
    _a: std::marker::PhantomData<S>,
}

impl<S: primitives::Integer> String<S>
{
    pub fn new(s: std::string::String) -> Self {
        String {
            value: s,
            _a: std::marker::PhantomData,
        }
    }
}

impl<S: primitives::Integer> Parcel for String<S>
{
    const TYPE_NAME: &'static str = "protocol::String<S>";

    fn read(read: &mut Read) -> Result<Self, Error> {
        let bytes = DynArray::<S, u8>::read(read)?;

        Ok(String::new(std::string::String::from_utf8(bytes.elements)?))
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        let array: DynArray<S, u8> = DynArray::new(self.value.bytes().collect());
        array.write(write)
    }
}

