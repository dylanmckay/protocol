macro_rules! impl_list_type {
    ( $ty:ident => T: $( $ty_pred:ident ),* ) => {
        impl<T> $crate::Parcel for ::std::collections::$ty<T>
            where T: $crate::Parcel $( + $ty_pred )*
        {
            const TYPE_NAME: &'static str = stringify!($ty<T>);

            fn read_field(read: &mut ::std::io::Read,
                          settings: &crate::Settings,
                          hints: &mut crate::hint::Hints) -> Result<Self, $crate::Error> {
                let elements = crate::util::read_list(read, settings, hints)?;
                Ok(elements.into_iter().collect())
            }

            fn write_field(&self, write: &mut ::std::io::Write,
                           settings: &crate::Settings,
                           hints: &mut crate::hint::Hints)
                -> Result<(), $crate::Error> {
                crate::util::write_list(self.iter(), write, settings, hints)
            }
        }

        #[cfg(test)]
        mod test
        {
            pub use crate::{Parcel, Settings};
            pub use std::collections::$ty;

            #[test]
            fn can_be_written_and_read_back_correctly() {
                let original: $ty<u32> = [1, 2, 3, 4, 5].iter().cloned().collect();

                let settings = Settings::default();
                let raw_bytes = original.raw_bytes(&settings).unwrap();
                let read_deque = $ty::<u32>::from_raw_bytes(&raw_bytes, &settings).unwrap();

                assert_eq!(original, read_deque);
            }
        }
    }
}

pub mod linked_list { impl_list_type!(LinkedList => T: ); }
pub mod vec_deque   { impl_list_type!(VecDeque   => T: ); }

pub mod btree_set   { impl_list_type!(BTreeSet   => T: Ord); }

pub mod hash_set {
    use std::hash::Hash;
    impl_list_type!(HashSet => T: Hash, Eq);
}

