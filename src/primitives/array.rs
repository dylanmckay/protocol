use {primitives, Parcel, Error};

use std::io::prelude::*;
use std;

/// The integer type that we will use to send length prefixes.
pub type SizeType = u32;

/// An dynamic array type with a custom size prefix type.
#[derive(Clone, Debug, PartialEq)]
pub struct DynArray<S: primitives::Integer, T: Parcel>
{
    pub elements: Vec<T>,
    _a: std::marker::PhantomData<S>,
}

impl<S: primitives::Integer, T: Parcel> DynArray<S,T>
{
    pub fn new(elements: Vec<T>) -> Self {
        DynArray { elements: elements, _a: std::marker::PhantomData }
    }
}

impl<S: primitives::Integer, T: Parcel> Parcel for DynArray<S, T>
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let size = S::read(read)?;
        let size: usize = size.try_into()?;

        let mut elements = Vec::new();

        for _ in 0..size {
            elements.push(T::read(read)?);
        }

        Ok(Self::new(elements))
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        S::try_from(self.elements.len())?.write(write)?;

        for element in self.elements.iter() {
            element.write(write)?;
        }

        Ok(())
    }
}

impl<T: Parcel> Parcel for Vec<T>
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(DynArray::<SizeType, T>::read(read)?.elements)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        DynArray::<SizeType, T>::new(self.clone()).write(write)
    }
}


