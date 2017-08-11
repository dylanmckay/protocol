use std;

error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    foreign_links {
        Io(std::io::Error);
        FromUtf8(std::string::FromUtf8Error);
        TryFromIntError(std::num::TryFromIntError);
        CharTryFromError(std::char::CharTryFromError);

        UuidParseError(::uuid::ParseError) #[cfg(feature = "uuid")];
    }

    errors {
        UnknownPacketId {
            description("unknown packet identifier")
            display("unknown packet identifier")
        }
    }
}

