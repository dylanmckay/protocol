use std::ffi::CString;
use std::io::prelude::{Read, Write};
use {hint, Error, Parcel, Settings};

impl Parcel for CString {
    const TYPE_NAME: &'static str = "CString";

    fn read_field(
        read: &mut dyn Read,
        settings: &Settings,
        _hints: &mut hint::Hints,
    ) -> Result<Self, Error> {
        let mut result = Vec::new();
        loop {
            let c: u8 = Parcel::read(read, settings)?;
            if c == 0x00 {
                break;
            }
            result.push(c);
        }
        Ok(CString::new(result)?)
    }

    fn write_field(
        &self,
        write: &mut dyn Write,
        settings: &Settings,
        _hints: &mut hint::Hints,
    ) -> Result<(), Error> {
        for c in self.clone().into_bytes() {
            c.write(write, settings)?;
            if c == 0x00 {
                return Ok(());
            }
        }
        0u8.write(write, settings)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use {Parcel, Settings};
    use std::io::Cursor;
    use std::ffi::CString;

    #[test]
    fn can_read_cstring() {
        let mut data = Cursor::new([0x41, 0x42, 0x43, 0]);
        let read_back: CString = Parcel::read(&mut data, &Settings::default()).unwrap();
        assert_eq!(read_back, CString::new("ABC").unwrap());
    }

    #[test]
    fn can_write_cstring() {
        let mut buffer = Cursor::new(Vec::new());

        CString::new("ABC").unwrap().write(&mut buffer, &Settings::default()).unwrap();
        assert_eq!(buffer.into_inner(), vec![0x41, 0x42, 0x43, 0]);
    }
}