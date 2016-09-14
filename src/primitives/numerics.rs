use {Parcel, Error, ByteOrder};

use std::io::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::num::TryFromIntError;

use byteorder::{ReadBytesExt, WriteBytesExt};

pub trait Integer : Parcel + TryFrom<u8, Err=TryFromIntError> + TryFrom<i8, Err=TryFromIntError> +
                    TryFrom<u16, Err=TryFromIntError> + TryFrom<i16, Err=TryFromIntError> +
                    TryFrom<u32, Err=TryFromIntError> + TryFrom<i32, Err=TryFromIntError> +
                    TryFrom<u64, Err=TryFromIntError> + TryFrom<i64, Err=TryFromIntError> +
                    TryFrom<usize, Err=TryFromIntError> + TryFrom<isize, Err=TryFromIntError> +
                    TryInto<u8, Err=TryFromIntError> + TryInto<i8, Err=TryFromIntError> +
                    TryInto<u16, Err=TryFromIntError> + TryInto<i16, Err=TryFromIntError> +
                    TryInto<u32, Err=TryFromIntError> + TryInto<i32, Err=TryFromIntError> +
                    TryInto<u64, Err=TryFromIntError> + TryInto<i64, Err=TryFromIntError> +
                    TryInto<usize, Err=TryFromIntError> + TryInto<isize, Err=TryFromIntError>
{

}

impl Parcel for bool
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        if read.read_u8()? == 0 { Ok(false) } else { Ok(true) }
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl Parcel for u8
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_u8()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_u8(*self)?; Ok(()) }
}

impl Parcel for i8
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_i8()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_i8(*self)?; Ok(()) }
}

impl Parcel for u16
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_u16::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_u16::<ByteOrder>(*self)?; Ok(()) }
}

impl Parcel for i16
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_i16::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_i16::<ByteOrder>(*self)?; Ok(()) }
}

impl Parcel for u32
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_u32::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_u32::<ByteOrder>(*self)?; Ok(()) }
}

impl Parcel for i32
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_i32::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_i32::<ByteOrder>(*self)?; Ok(()) }
}

impl Parcel for u64
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_u64::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_u64::<ByteOrder>(*self)?; Ok(()) }
}

impl Parcel for i64
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_i64::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_i64::<ByteOrder>(*self)?; Ok(()) }
}

impl Parcel for f32
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_f32::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_f32::<ByteOrder>(*self)?; Ok(()) }
}

impl Parcel for f64
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_f64::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_f64::<ByteOrder>(*self)?; Ok(()) }
}

impl Integer for u8 { }
impl Integer for i8 { }
impl Integer for u16 { }
impl Integer for i16 { }
impl Integer for u32 { }
impl Integer for i32 { }
impl Integer for u64 { }
impl Integer for i64 { }

