use {Parcel, Error, Settings};

use std::io::prelude::*;

use num_traits::{FromPrimitive, ToPrimitive};
use byteorder::{ReadBytesExt, WriteBytesExt};

/// An integer value that can be serialized and deserialized.
pub trait Integer : Parcel + FromPrimitive + ToPrimitive { }

impl Parcel for bool
{
    const TYPE_NAME: &'static str = "bool";

    fn read(read: &mut Read,
            _: &Settings) -> Result<Self, Error> {
        if read.read_u8()? == 0 { Ok(false) } else { Ok(true) }
    }

    fn write(&self, write: &mut Write,
             _: &Settings) -> Result<(), Error> {
        write.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl Parcel for u8
{
    const TYPE_NAME: &'static str = "u8";

    fn read(read: &mut Read,
            _: &Settings) -> Result<Self, Error> { Ok(read.read_u8()?) }
    fn write(&self, write: &mut Write,
             _: &Settings) -> Result<(), Error> { write.write_u8(*self)?; Ok(()) }
}

impl Parcel for i8
{
    const TYPE_NAME: &'static str = "i8";

    fn read(read: &mut Read,
            _: &Settings) -> Result<Self, Error> { Ok(read.read_i8()?) }
    fn write(&self, write: &mut Write,
             _: &Settings) -> Result<(), Error> { write.write_i8(*self)?; Ok(()) }
}

macro_rules! impl_parcel_for_numeric {
    ($ty:ident => [$read_fn:ident : $write_fn:ident]) => {
        impl Parcel for $ty {
            const TYPE_NAME: &'static str = stringify!($ty);

            fn read(read: &mut Read,
                    settings: &Settings) -> Result<Self, Error> {
                Ok(settings.byte_order.$read_fn(read)?)
            }

            fn write(&self, write: &mut Write,
                     settings: &Settings) -> Result<(), Error> {
                settings.byte_order.$write_fn(*self, write)?; Ok(())
            }
        }
    };
}

impl_parcel_for_numeric!(u16 => [read_u16 : write_u16]);
impl_parcel_for_numeric!(i16 => [read_i16 : write_i16]);
impl_parcel_for_numeric!(u32 => [read_u32 : write_u32]);
impl_parcel_for_numeric!(i32 => [read_i32 : write_i32]);
impl_parcel_for_numeric!(u64 => [read_u64 : write_u64]);
impl_parcel_for_numeric!(i64 => [read_i64 : write_i64]);
impl_parcel_for_numeric!(f32 => [read_f32 : write_f32]);
impl_parcel_for_numeric!(f64 => [read_f64 : write_f64]);

impl Integer for u8 { }
impl Integer for i8 { }
impl Integer for u16 { }
impl Integer for i16 { }
impl Integer for u32 { }
impl Integer for i32 { }
impl Integer for u64 { }
impl Integer for i64 { }

