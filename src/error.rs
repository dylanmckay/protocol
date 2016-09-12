use std;

#[derive(Debug)]
pub enum Error
{
    UnknownPacketId,
    Io(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
    TryFromIntError(std::num::TryFromIntError),
}

impl From<std::io::Error> for Error
{
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for Error
{
    fn from(e: std::string::FromUtf8Error) -> Error {
        Error::FromUtf8(e)
    }
}

impl From<std::num::TryFromIntError> for Error
{
    fn from(e: std::num::TryFromIntError) -> Error {
        Error::TryFromIntError(e)
    }
}

