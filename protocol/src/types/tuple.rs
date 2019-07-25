use crate::{Parcel, Error, Settings};
use crate::hint;

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

impl<T0, T1, T2, T3, T4, T5> Parcel for (T0, T1, T2, T3, T4, T5)
    where T0: Parcel, T1: Parcel, T2: Parcel, T3: Parcel, T4: Parcel,
          T5: Parcel,
{
    const TYPE_NAME: &'static str = "(T0, T1, T2, T3, T4, T5)";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let v0 = T0::read(read, settings)?;
        let v1 = T1::read(read, settings)?;
        let v2 = T2::read(read, settings)?;
        let v3 = T3::read(read, settings)?;
        let v4 = T4::read(read, settings)?;
        let v5 = T5::read(read, settings)?;
        Ok((v0, v1, v2, v3, v4, v5))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.0.write(write, settings)?;
        self.1.write(write, settings)?;
        self.2.write(write, settings)?;
        self.3.write(write, settings)?;
        self.4.write(write, settings)?;
        self.5.write(write, settings)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> Parcel for (T0, T1, T2, T3, T4, T5, T6)
    where T0: Parcel, T1: Parcel, T2: Parcel, T3: Parcel, T4: Parcel,
          T5: Parcel, T6: Parcel,
{
    const TYPE_NAME: &'static str = "(T0, T1, T2, T3, T4, T5, T6)";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let v0 = T0::read(read, settings)?;
        let v1 = T1::read(read, settings)?;
        let v2 = T2::read(read, settings)?;
        let v3 = T3::read(read, settings)?;
        let v4 = T4::read(read, settings)?;
        let v5 = T5::read(read, settings)?;
        let v6 = T6::read(read, settings)?;
        Ok((v0, v1, v2, v3, v4, v5, v6))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.0.write(write, settings)?;
        self.1.write(write, settings)?;
        self.2.write(write, settings)?;
        self.3.write(write, settings)?;
        self.4.write(write, settings)?;
        self.5.write(write, settings)?;
        self.6.write(write, settings)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Parcel for (T0, T1, T2, T3, T4, T5, T6, T7)
    where T0: Parcel, T1: Parcel, T2: Parcel, T3: Parcel, T4: Parcel,
          T5: Parcel, T6: Parcel, T7: Parcel,
{
    const TYPE_NAME: &'static str = "(T0, T1, T2, T3, T4, T5, T6, T7)";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let v0 = T0::read(read, settings)?;
        let v1 = T1::read(read, settings)?;
        let v2 = T2::read(read, settings)?;
        let v3 = T3::read(read, settings)?;
        let v4 = T4::read(read, settings)?;
        let v5 = T5::read(read, settings)?;
        let v6 = T6::read(read, settings)?;
        let v7 = T7::read(read, settings)?;
        Ok((v0, v1, v2, v3, v4, v5, v6, v7))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.0.write(write, settings)?;
        self.1.write(write, settings)?;
        self.2.write(write, settings)?;
        self.3.write(write, settings)?;
        self.4.write(write, settings)?;
        self.5.write(write, settings)?;
        self.6.write(write, settings)?;
        self.7.write(write, settings)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Parcel for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
    where T0: Parcel, T1: Parcel, T2: Parcel, T3: Parcel, T4: Parcel,
          T5: Parcel, T6: Parcel, T7: Parcel, T8: Parcel,
{
    const TYPE_NAME: &'static str = "(T0, T1, T2, T3, T4, T5, T6, T7, T8)";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let v0 = T0::read(read, settings)?;
        let v1 = T1::read(read, settings)?;
        let v2 = T2::read(read, settings)?;
        let v3 = T3::read(read, settings)?;
        let v4 = T4::read(read, settings)?;
        let v5 = T5::read(read, settings)?;
        let v6 = T6::read(read, settings)?;
        let v7 = T7::read(read, settings)?;
        let v8 = T8::read(read, settings)?;
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.0.write(write, settings)?;
        self.1.write(write, settings)?;
        self.2.write(write, settings)?;
        self.3.write(write, settings)?;
        self.4.write(write, settings)?;
        self.5.write(write, settings)?;
        self.6.write(write, settings)?;
        self.7.write(write, settings)?;
        self.8.write(write, settings)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Parcel for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
    where T0: Parcel, T1: Parcel, T2: Parcel, T3: Parcel, T4: Parcel,
          T5: Parcel, T6: Parcel, T7: Parcel, T8: Parcel, T9: Parcel,
{
    const TYPE_NAME: &'static str = "(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let v0 = T0::read(read, settings)?;
        let v1 = T1::read(read, settings)?;
        let v2 = T2::read(read, settings)?;
        let v3 = T3::read(read, settings)?;
        let v4 = T4::read(read, settings)?;
        let v5 = T5::read(read, settings)?;
        let v6 = T6::read(read, settings)?;
        let v7 = T7::read(read, settings)?;
        let v8 = T8::read(read, settings)?;
        let v9 = T9::read(read, settings)?;
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.0.write(write, settings)?;
        self.1.write(write, settings)?;
        self.2.write(write, settings)?;
        self.3.write(write, settings)?;
        self.4.write(write, settings)?;
        self.5.write(write, settings)?;
        self.6.write(write, settings)?;
        self.7.write(write, settings)?;
        self.8.write(write, settings)?;
        self.9.write(write, settings)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Parcel for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
    where T0: Parcel, T1: Parcel, T2: Parcel, T3: Parcel, T4: Parcel,
          T5: Parcel, T6: Parcel, T7: Parcel, T8: Parcel, T9: Parcel,
          T10: Parcel,
{
    const TYPE_NAME: &'static str = "(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)";

    fn read_field(read: &mut Read,
                  settings: &Settings,
                  _: &mut hint::Hints) -> Result<Self, Error> {
        let v0 = T0::read(read, settings)?;
        let v1 = T1::read(read, settings)?;
        let v2 = T2::read(read, settings)?;
        let v3 = T3::read(read, settings)?;
        let v4 = T4::read(read, settings)?;
        let v5 = T5::read(read, settings)?;
        let v6 = T6::read(read, settings)?;
        let v7 = T7::read(read, settings)?;
        let v8 = T8::read(read, settings)?;
        let v9 = T9::read(read, settings)?;
        let v10 = T10::read(read, settings)?;
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10))
    }

    fn write_field(&self, write: &mut Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.0.write(write, settings)?;
        self.1.write(write, settings)?;
        self.2.write(write, settings)?;
        self.3.write(write, settings)?;
        self.4.write(write, settings)?;
        self.5.write(write, settings)?;
        self.6.write(write, settings)?;
        self.7.write(write, settings)?;
        self.8.write(write, settings)?;
        self.9.write(write, settings)?;
        self.10.write(write, settings)?;

        Ok(())
    }
}
