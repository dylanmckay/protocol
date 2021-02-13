use crate::{hint, util, Error, Parcel, Settings};
use std::ffi::CString;
use std::io::prelude::{Read, Write};

impl Parcel for CString {
    const TYPE_NAME: &'static str = "CString";

    fn read_field(
        read: &mut dyn Read,
        settings: &Settings,
        _hints: &mut hint::Hints,
    ) -> Result<Self, Error> {
        let mut result = Vec::new();
        // this logic is susceptible to DoS attacks by never providing
        //   a null character and will be fixed by
        //   https://github.com/dylanmckay/protocol/issues/14
        loop {
            let c: u8 = Parcel::read(read, settings)?;
            if c == 0x00 {
                return Ok(CString::new(result)?);
            }
            result.push(c);
        }
    }

    fn write_field(
        &self,
        write: &mut dyn Write,
        settings: &Settings,
        _hints: &mut hint::Hints,
    ) -> Result<(), Error> {
        util::write_items(self.clone().into_bytes_with_nul().iter(), write, settings)
    }
}

#[cfg(test)]
mod test {
    use crate::{Parcel, Settings};
    use std::ffi::CString;
    use std::io::Cursor;

    #[test]
    fn can_read_cstring() {
        let mut data = Cursor::new([0x41, 0x42, 0x43, 0]);
        let read_back: CString = Parcel::read(&mut data, &Settings::default()).unwrap();
        assert_eq!(read_back, CString::new("ABC").unwrap());
    }

    #[test]
    fn can_write_cstring() {
        let mut buffer = Cursor::new(Vec::new());

        CString::new("ABC")
            .unwrap()
            .write(&mut buffer, &Settings::default())
            .unwrap();
        assert_eq!(buffer.into_inner(), vec![0x41, 0x42, 0x43, 0]);
    }
}
