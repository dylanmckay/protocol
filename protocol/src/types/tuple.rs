use {Parcel, Error, Settings};
use hint;

use std::io::prelude::*;

impl<T0, T1> Parcel for (T0, T1)
    where T0: Parcel, T1: Parcel
{
    const TYPE_NAME: &'static str = "(T0, T1)";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let v0 = T0::read(read, settings)?;
        let v1 = T1::read(read, settings)?;
        Ok((v0, v1))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.0.write(write, settings)?;
        self.1.write(write, settings)?;

        Ok(())
    }
}

impl<T0, T1, T2> Parcel for (T0, T1, T2)
    where T0: Parcel, T1: Parcel, T2: Parcel
{
    const TYPE_NAME: &'static str = "(T0, T1, T2)";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let v0 = T0::read(read, settings)?;
        let v1 = T1::read(read, settings)?;
        let v2 = T2::read(read, settings)?;
        Ok((v0, v1, v2))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.0.write(write, settings)?;
        self.1.write(write, settings)?;
        self.2.write(write, settings)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3> Parcel for (T0, T1, T2, T3)
    where T0: Parcel, T1: Parcel, T2: Parcel, T3: Parcel
{
    const TYPE_NAME: &'static str = "(T0, T1, T2, T3)";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let v0 = T0::read(read, settings)?;
        let v1 = T1::read(read, settings)?;
        let v2 = T2::read(read, settings)?;
        let v3 = T3::read(read, settings)?;
        Ok((v0, v1, v2, v3))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.0.write(write, settings)?;
        self.1.write(write, settings)?;
        self.2.write(write, settings)?;
        self.3.write(write, settings)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4> Parcel for (T0, T1, T2, T3, T4)
    where T0: Parcel, T1: Parcel, T2: Parcel, T3: Parcel, T4: Parcel
{
    const TYPE_NAME: &'static str = "(T0, T1, T2, T3, T4)";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let v0 = T0::read(read, settings)?;
        let v1 = T1::read(read, settings)?;
        let v2 = T2::read(read, settings)?;
        let v3 = T3::read(read, settings)?;
        let v4 = T4::read(read, settings)?;
        Ok((v0, v1, v2, v3, v4))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.0.write(write, settings)?;
        self.1.write(write, settings)?;
        self.2.write(write, settings)?;
        self.3.write(write, settings)?;
        self.4.write(write, settings)?;

        Ok(())
    }
}

