use {Type, Error};

use std::io::prelude::*;

impl<T0, T1> Type for (T0, T1)
    where T0: Type, T1: Type
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let v0 = T0::read(read)?;
        let v1 = T1::read(read)?;
        Ok((v0, v1))
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        self.0.write(write)?;
        self.1.write(write)?;

        Ok(())
    }
}

impl<T0, T1, T2> Type for (T0, T1, T2)
    where T0: Type, T1: Type, T2: Type
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let v0 = T0::read(read)?;
        let v1 = T1::read(read)?;
        let v2 = T2::read(read)?;
        Ok((v0, v1, v2))
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        self.0.write(write)?;
        self.1.write(write)?;
        self.2.write(write)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3> Type for (T0, T1, T2, T3)
    where T0: Type, T1: Type, T2: Type, T3: Type
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let v0 = T0::read(read)?;
        let v1 = T1::read(read)?;
        let v2 = T2::read(read)?;
        let v3 = T3::read(read)?;
        Ok((v0, v1, v2, v3))
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        self.0.write(write)?;
        self.1.write(write)?;
        self.2.write(write)?;
        self.3.write(write)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4> Type for (T0, T1, T2, T3, T4)
    where T0: Type, T1: Type, T2: Type, T3: Type, T4: Type
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let v0 = T0::read(read)?;
        let v1 = T1::read(read)?;
        let v2 = T2::read(read)?;
        let v3 = T3::read(read)?;
        let v4 = T4::read(read)?;
        Ok((v0, v1, v2, v3, v4))
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        self.0.write(write)?;
        self.1.write(write)?;
        self.2.write(write)?;
        self.3.write(write)?;
        self.4.write(write)?;

        Ok(())
    }
}

