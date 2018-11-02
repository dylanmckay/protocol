use {primitives, Parcel, Error};

use std::io::prelude::*;
use std;

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
    const TYPE_NAME: &'static str = "DynArray<S,T>";

    fn read(read: &mut Read) -> Result<Self, Error> {
        let elements = primitives::util::read_list_ext::<S,T>(read)?;
        Ok(Self::new(elements))
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        primitives::util::write_list_ext::<S,T,_>(write, self.elements.iter())
    }
}

macro_rules! impl_parcel_for_array {
    ($n:expr) => {
        impl<T: Parcel> Parcel for [T; $n] where T: Copy {
            const TYPE_NAME: &'static str = stringify!([T; $n]);

            fn read(read: &mut Read) -> Result<Self, Error> {
                use std::mem;

                let mut elements: Vec<T> = Vec::with_capacity($n);

                for _ in 0..$n {
                    let elem = T::read(read)?;
                    elements.push(elem);
                }

                let mut array: [T; $n] = unsafe { mem::uninitialized() };
                array.clone_from_slice(&elements[..]);

                Ok(array)
            }

            fn write(&self, write: &mut Write) -> Result<(), Error> {
                for elem in self.iter() {
                    elem.write(write)?;
                }

                Ok(())
            }
        }
    }
}

impl_parcel_for_array!(1);
impl_parcel_for_array!(2);
impl_parcel_for_array!(3);
impl_parcel_for_array!(4);
impl_parcel_for_array!(5);
impl_parcel_for_array!(6);
impl_parcel_for_array!(7);
impl_parcel_for_array!(8);
impl_parcel_for_array!(9);
impl_parcel_for_array!(10);
impl_parcel_for_array!(11);
impl_parcel_for_array!(12);
impl_parcel_for_array!(13);
impl_parcel_for_array!(14);
impl_parcel_for_array!(15);
impl_parcel_for_array!(16);
impl_parcel_for_array!(17);
impl_parcel_for_array!(18);
impl_parcel_for_array!(19);
impl_parcel_for_array!(20);
impl_parcel_for_array!(21);
impl_parcel_for_array!(22);
impl_parcel_for_array!(23);
impl_parcel_for_array!(24);
impl_parcel_for_array!(25);
impl_parcel_for_array!(26);
impl_parcel_for_array!(27);
impl_parcel_for_array!(28);
impl_parcel_for_array!(29);
impl_parcel_for_array!(30);
impl_parcel_for_array!(31);
impl_parcel_for_array!(32);
impl_parcel_for_array!(40);
impl_parcel_for_array!(42);
impl_parcel_for_array!(48);
impl_parcel_for_array!(64);
impl_parcel_for_array!(80);
impl_parcel_for_array!(120);
impl_parcel_for_array!(128);
impl_parcel_for_array!(256);
impl_parcel_for_array!(500);
impl_parcel_for_array!(512);
impl_parcel_for_array!(1000);
impl_parcel_for_array!(1024);
impl_parcel_for_array!(4096);
impl_parcel_for_array!(0xffff);

impl<T: Parcel> Parcel for Vec<T>
{
    const TYPE_NAME: &'static str = "Vec<T>";

    fn read(read: &mut Read) -> Result<Self, Error> {
        primitives::util::read_list(read)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        primitives::util::write_list(write, self.iter())
    }
}

#[cfg(test)]
mod test {
    use Parcel;
    use std::io::Cursor;

    #[test]
    fn can_read_array() {
        let mut data = Cursor::new([0u8, 1, 2, 3]);
        let read_back: [u8; 4] = Parcel::read(&mut data).unwrap();
        assert_eq!(read_back, [0, 1, 2, 3]);
    }

    #[test]
    fn can_write_array() {
        let mut buffer = Cursor::new(Vec::new());
        [5u8, 7, 9, 11].write(&mut buffer).unwrap();
        assert_eq!(buffer.into_inner(), vec![5, 7, 9, 11]);
    }
}
