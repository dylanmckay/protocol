#[macro_export]
macro_rules! define_middleware_pipeline {
    ($ty:ident { $( $mw_name:ident : $mw_ty:ty ),+ } ) => {
        pub struct $ty
        {
            $( pub $mw_name : $mw_ty),+
        }

        impl $ty
        {
            pub fn encode_data(&mut self, mut data: Vec<u8>)
                -> Result<Vec<u8>, $crate::Error> {
                use $crate::wire::Middleware;

                $( data = self.$mw_name.encode_data(data)?; )+

                Ok(data)
            }

            pub fn decode_data(&mut self, mut data: Vec<u8>)
                -> Result<Vec<u8>, $crate::Error> {
                for middleware in self.middleware_mut() {
                    data = middleware.decode_data(data)?;
                }

                Ok(data)
            }

            /// Gets the middleware pipeline.
            pub fn middleware_mut(&mut self) -> ::std::collections::VecDeque<&mut $crate::wire::Middleware> {
                let mut middleware = ::std::collections::VecDeque::new();

                $( middleware.push_front(&mut self.$mw_name as &mut $crate::wire::Middleware); )+

                middleware
            }
        }
    }
}

#[cfg(test)]
mod test
{
    use Error;
    use wire;

    define_middleware_pipeline!(Pipeline {
        encryption: NullMiddleware,
        compression: NullMiddleware
    });

    pub struct NullMiddleware;

    impl wire::Middleware for NullMiddleware
    {
        fn encode_data(&mut self, data: Vec<u8>) -> Result<Vec<u8>, Error> { Ok(data) }
        fn decode_data(&mut self, data: Vec<u8>) -> Result<Vec<u8>, Error> { Ok(data) }
    }

    describe! pipeline {
        before_each {
            let mut null_pipeline = Pipeline {
                encryption: NullMiddleware,
                compression: NullMiddleware,
            };

            let data = vec![7, 2, 5, 5, 1, 2, 3, 4, 2, 4, 8];
        }

        it "successfully passes data through the pipeline" {
            assert_eq!(null_pipeline.encode_data(data.clone()).unwrap(), data.clone());
            assert_eq!(null_pipeline.decode_data(data.clone()).unwrap(), data.clone());
        }
    }
}
