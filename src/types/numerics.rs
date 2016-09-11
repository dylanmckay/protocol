use {Type, Error, ByteOrder};

use std::io::prelude::*;

use byteorder::{ReadBytesExt, WriteBytesExt};

impl Type for bool
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        if read.read_u8()? == 0 { Ok(false) } else { Ok(true) }
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl Type for u8
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_u8()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_u8(*self)?; Ok(()) }
}

impl Type for i8
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_i8()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_i8(*self)?; Ok(()) }
}

impl Type for u16
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_u16::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_u16::<ByteOrder>(*self)?; Ok(()) }
}

impl Type for i16
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_i16::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_i16::<ByteOrder>(*self)?; Ok(()) }
}

impl Type for u32
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_u32::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_u32::<ByteOrder>(*self)?; Ok(()) }
}

impl Type for i32
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_i32::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_i32::<ByteOrder>(*self)?; Ok(()) }
}

impl Type for u64
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_u64::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_u64::<ByteOrder>(*self)?; Ok(()) }
}

impl Type for i64
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_i64::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_i64::<ByteOrder>(*self)?; Ok(()) }
}

impl Type for f32
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_f32::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_f32::<ByteOrder>(*self)?; Ok(()) }
}

impl Type for f64
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_f64::<ByteOrder>()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_f64::<ByteOrder>(*self)?; Ok(()) }
}

