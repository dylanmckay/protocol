use {Parcel, Error};
use std::io::prelude::*;

use uuid::Uuid;

#[cfg(feature = "tokio")]
use tokio::prelude::*;
#[cfg(feature = "tokio")]
use tokio;

impl Parcel for Uuid
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let bytes: Result<Vec<u8>, _> = read.bytes().take(16).collect();
        let bytes = bytes?;

        Ok(Uuid::from_bytes(&bytes)?)
    }

    #[cfg(feature = "tokio")]
    fn read_async(read: &mut AsyncRead) -> Box<Future<Item=Self, Error=Error> + Send> {
        Box::new(tokio::io::read_exact(read, [0; 16])
            .and_then(bytes| Uuid::from_bytes(&bytes)))
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write(self.as_bytes())?;
        Ok(())
    }
}

