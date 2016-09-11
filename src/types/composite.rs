/// Defines a type built out of other types.
#[macro_export]
macro_rules! define_composite_type {
    ($ty:ident { $( $field_name:ident : $field_ty:ty ),+ }) => {
        #[derive(Clone,Debug)]
        pub struct $ty
        {
            $( $field_name : $field_ty ),+
        }

        impl $crate::Type for $ty
        {
            fn read(read: &mut ::std::io::Read) -> Result<Self, $crate::Error> {
                Ok($ty {
                    $( $field_name: <$field_ty as $crate::Type>::read(read)? ),+
                })
            }

            fn write(&self, write: &mut ::std::io::Write) -> Result<(), $crate::Error> {
                $( self.$field_name.write(write)?; )+

                Ok(())
            }
        }
    }
}

