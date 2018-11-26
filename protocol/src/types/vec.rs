use {Parcel, Error, Settings};
use {hint, types, util};
use std::io::prelude::*;
use std;

/// A newtype wrapping `Vec<T>` but with a custom length prefix type.
#[derive(Clone, Debug, PartialEq)]
pub struct Vec<S: types::Integer, T: Parcel>
{
    /// The inner `Vec<T>`.
    pub elements: std::vec::Vec<T>,
    _a: std::marker::PhantomData<S>,
}

impl<S: types::Integer, T: Parcel> Vec<S,T>
{
    /// Creates a new `Vec` from a list of elements.
    pub fn new(elements: std::vec::Vec<T>) -> Self {
        Vec { elements: elements, _a: std::marker::PhantomData }
    }
}

impl<S: types::Integer, T: Parcel> Parcel for Vec<S, T>
{
    const TYPE_NAME: &'static str = "protocol::Vec<S,T>";

    fn read(read: &mut Read,
            settings: &Settings,
            hints: &mut hint::Hints) -> Result<Self, Error> {
        let elements = util::read_list_ext::<S,T>(read, settings, hints)?;
        Ok(Self::new(elements))
    }

    fn write(&self, write: &mut Write,
             settings: &Settings) -> Result<(), Error> {
        util::write_list_ext::<S,T,_>(write, self.elements.iter(), settings)
    }
}

/// Stuff relating to `std::vec::Vec<T>`.
mod std_vec {
    use {Error, Parcel, Settings};
    use {hint, util};
    use std::io::prelude::*;

    impl<T: Parcel> Parcel for Vec<T>
    {
        const TYPE_NAME: &'static str = "Vec<T>";

        fn read(read: &mut Read,
                settings: &Settings,
                hints: &mut hint::Hints) -> Result<Self, Error> {
            util::read_list(read, settings, hints)
        }

        fn write(&self, write: &mut Write,
                 settings: &Settings) -> Result<(), Error> {
            util::write_list(write, self.iter(), settings)
        }
    }
}

