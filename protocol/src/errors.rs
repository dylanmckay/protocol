use std::{self, fmt, error};

macro_rules! from_error {
    ($f: ty, $e: expr) => {
        impl From<$f> for Error {
            fn from(f: $f) -> Error {
                $e(f)
            }
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Copy of [TryFromIntError](https://doc.rust-lang.org/std/num/struct.TryFromIntError.html)
/// that works in stable rust
pub struct TryFromIntError { }

impl TryFromIntError {
    fn description(&self) -> &str {
        "out of range integral type conversion attempted"
    }
}

impl fmt::Display for TryFromIntError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(fmt)
    }
}

impl error::Error for TryFromIntError {
    fn description(&self) -> &str {
        self.description()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Copy of [CharTryFromError](https://doc.rust-lang.org/std/char/struct.CharTryFromError.html)
/// that works in stable rust
pub struct CharTryFromError { }

impl CharTryFromError {
    fn description(&self) -> &str {
         "converted integer out of range for `char`"
    }
}

impl fmt::Display for CharTryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       self.description().fmt(f)
    }
}

impl error::Error for CharTryFromError {
    fn description(&self) -> &str {
        self.description()
    }
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
    TryFromIntError(TryFromIntError),
    CharTryFromError(CharTryFromError),
    #[cfg(feature = "uuid")]
    UuidParseError(::uuid::parser::ParseError),
    UnknownPacketId,
    UnimplementedParcel(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => writeln!(fmt, "{}", err),
            Error::FromUtf8(ref err) => writeln!(fmt, "{}", err),
            Error::TryFromIntError(ref err) => writeln!(fmt, "{}", err),
            Error::CharTryFromError(ref err) => writeln!(fmt, "{}", err),
            #[cfg(feature = "uuid")]
            Error::UuidParseError(ref err) => writeln!(fmt, "{}", err),
            Error::UnknownPacketId => writeln!(fmt, "unknown packet identifier"),
            Error::UnimplementedParcel(ref err) => writeln!(fmt, "unimplemented parcel type: {}", err),
        }
    }
}

from_error!(std::io::Error, Error::Io);
from_error!(std::string::FromUtf8Error, Error::FromUtf8);
from_error!(TryFromIntError, Error::TryFromIntError);
from_error!(CharTryFromError, Error::CharTryFromError);
#[cfg(feature = "uuid")]
from_error!(::uuid::parser::ParseError, Error::UuidParseError);

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::FromUtf8(ref err) => Some(err),
            Error::TryFromIntError(ref err) => Some(err),
            Error::CharTryFromError(ref err) => Some(err),
            #[cfg(feature = "uuid")]
            Error::UuidParseError(ref err) => Some(err),
            _ => None,
        }
    }
}