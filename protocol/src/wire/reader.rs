use crate::{Error, Parcel, Settings};
use std::io;
use std::io::prelude::*;

/// A receive buffer that waits until enough data is ready
/// and then returns the parsed parcels.
///
/// This mechanism can be used to send parcels without
/// length prefixes.
///
///
/// # Adding data to the receive buffer
///
/// `Reader` implements `std::io::Write`. Any bytes written
/// to the reader are added to the receive queue for future
/// processing.
///
/// # Example
///
/// This example shows a reader only returning a `u32`
/// once enough data has been buffered.
///
/// ```
/// use protocol;
/// use std::io::Write;
///
/// let mut reader = protocol::wire::Reader::new();
/// let settings = protocol::Settings::default();
///
/// // No bytes received yet.
/// assert_eq!(None, reader.poll::<u32>(&settings).unwrap());
///
/// // Two bytes received.
/// reader.write(&[0xff, 0x00]).unwrap();
/// assert_eq!(None, reader.poll::<u32>(&settings).unwrap());
///
/// // Three bytes received.
/// reader.write(&[0x00]).unwrap();
/// assert_eq!(None, reader.poll::<u32>(&settings).unwrap());
///
/// // All four bytes received.
/// reader.write(&[0x00]).unwrap();
/// assert_eq!(Some(0xff000000), reader.poll::<u32>(&settings).unwrap());
/// ```
#[derive(Debug)]
pub struct Reader {
    /// The internal receive buffer.
    ///
    /// Contains all bytes that have been received but not yet parsed
    /// into a packet.
    receive_buffer: Vec<u8>,
}

impl Reader {
    /// Creates a new parcel reader.
    pub fn new() -> Self {
        Reader {
            receive_buffer: Vec::new(),
        }
    }

    /// Polls the reader for a value.
    ///
    /// Returns `Ok(None)` if further data must be received in order
    /// to interpret the value.
    ///
    /// Returns `Ok(Some(value))` if the value is ready to be read
    /// from the stream.
    ///
    /// Returns `Err(e)` on error.
    pub fn poll<P>(&mut self,
                   settings: &Settings)
        -> Result<Option<P>, Error>
        where P: Parcel {
        let mut cursor = io::Cursor::new(self.receive_buffer.clone());

        match Parcel::read(&mut cursor, settings) {
            Ok(value) => {
                // Remove the interpreted bytes from the receive buffer.
                let bytes_read = cursor.position() as usize;
                self.receive_buffer.drain(0..bytes_read);

                Ok(Some(value))
            },
            Err(e) => match e {
                Error::Io(io) => {
                    // Ignore errors caused by the receive buffer
                    // not having enough data yet.
                    if io.kind() == io::ErrorKind::UnexpectedEof {
                        Ok(None)
                    } else {
                        // An actual IO error.
                        Err(Error::Io(io).into())
                    }
                },
                _ => Err(e),
            },
        }
    }
}

impl Write for Reader {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.receive_buffer.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

impl Default for Reader {
    fn default() -> Self {
        Reader::new()
    }
}

