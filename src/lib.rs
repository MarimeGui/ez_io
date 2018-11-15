pub mod error;

use crate::error::{MagicNumberCheckError, WrongMagicNumber};
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::mem::transmute;
use std::result::Result as STDResult;

pub enum Endian {
    BigEndian,
    LittleEndian,
}

macro_rules! read_auto {
    ($endian:ident, $size:expr, $to:ty, $self:expr) => {{
        let mut temp: [u8; $size] = [0; $size];
        $self.read_exact(&mut temp)?;
        unsafe { Ok(<$to>::$endian(transmute::<[u8; $size], $to>(temp))) }
    }};
}

pub trait ReadE: Read {
    // u8
    fn read_to_u8(&mut self) -> Result<u8> {
        let mut temp: [u8; 1] = [0];
        self.read_exact(&mut temp)?;
        Ok(temp[0])
    }

    // u16
    fn read_to_u16(&mut self, endian: Endian) -> Result<u16> {
        match endian {
            Endian::BigEndian => self.read_be_to_u16(),
            Endian::LittleEndian => self.read_le_to_u16(),
        }
    }

    fn read_le_to_u16(&mut self) -> Result<u16> {
        read_auto!(from_le, 2, u16, self)
    }

    fn read_be_to_u16(&mut self) -> Result<u16> {
        read_auto!(from_be, 2, u16, self)
    }

    // u32
    fn read_to_u32(&mut self, endian: Endian) -> Result<u32> {
        match endian {
            Endian::BigEndian => self.read_be_to_u32(),
            Endian::LittleEndian => self.read_le_to_u32(),
        }
    }

    fn read_le_to_u32(&mut self) -> Result<u32> {
        read_auto!(from_le, 4, u32, self)
    }

    fn read_be_to_u32(&mut self) -> Result<u32> {
        read_auto!(from_be, 4, u32, self)
    }

    // i8
    fn read_to_i8(&mut self) -> Result<i8> {
        let mut temp: [u8; 1] = [0];
        self.read_exact(&mut temp[..])?;
        unsafe { Ok(transmute::<u8, i8>(temp[0])) }
    }

    // i16
    fn read_to_i16(&mut self, endian: Endian) -> Result<i16> {
        match endian {
            Endian::BigEndian => self.read_be_to_i16(),
            Endian::LittleEndian => self.read_le_to_i16(),
        }
    }

    fn read_le_to_i16(&mut self) -> Result<i16> {
        read_auto!(from_le, 2, i16, self)
    }

    fn read_be_to_i16(&mut self) -> Result<i16> {
        read_auto!(from_be, 2, i16, self)
    }

    // i32
    fn read_to_i32(&mut self, endian: Endian) -> Result<i32> {
        match endian {
            Endian::BigEndian => self.read_be_to_i32(),
            Endian::LittleEndian => self.read_le_to_i32(),
        }
    }

    fn read_le_to_i32(&mut self) -> Result<i32> {
        read_auto!(from_le, 4, i32, self)
    }

    fn read_be_to_i32(&mut self) -> Result<i32> {
        read_auto!(from_be, 4, i32, self)
    }

    // f32
    fn read_to_f32(&mut self, endian: Endian) -> Result<f32> {
        match endian {
            Endian::BigEndian => self.read_be_to_f32(),
            Endian::LittleEndian => self.read_le_to_f32(),
        }
    }

    fn read_le_to_f32(&mut self) -> Result<f32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 4], f32>(temp)) }
    }

    fn read_be_to_f32(&mut self) -> Result<f32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 4], f32>(temp)) }
    }

    // f64
    fn read_to_f64(&mut self, endian: Endian) -> Result<f64> {
        match endian {
            Endian::BigEndian => self.read_be_to_f64(),
            Endian::LittleEndian => self.read_le_to_f64(),
        }
    }

    fn read_le_to_f64(&mut self) -> Result<f64> {
        let mut temp: [u8; 8] = [0; 8];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 8], f64>(temp)) }
    }

    fn read_be_to_f64(&mut self) -> Result<f64> {
        let mut temp: [u8; 8] = [0; 8];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 8], f64>(temp)) }
    }
}

impl<R: Read> ReadE for R {}

macro_rules! write_auto {
    ($endian:ident, $size:expr, $from:ty, $tw:expr, $self:expr) => {{
        let temp = unsafe { transmute::<$from, [u8; $size]>($tw.$endian()) };
        $self.write_all(&temp)?;
        Ok(())
    }};
}

pub trait WriteE: Write {
    // u8
    fn write_to_u8(&mut self, tw: u8) -> Result<()> {
        self.write_all(&[tw])?;
        Ok(())
    }

    // u16
    fn write_le_to_u16(&mut self, tw: u16) -> Result<()> {
        write_auto!(to_le, 2, u16, tw, self)
    }

    fn write_be_to_u16(&mut self, tw: u16) -> Result<()> {
        write_auto!(to_be, 2, u16, tw, self)
    }

    // u32
    fn write_le_to_u32(&mut self, tw: u32) -> Result<()> {
        write_auto!(to_le, 4, u32, tw, self)
    }

    fn write_be_to_u32(&mut self, tw: u32) -> Result<()> {
        write_auto!(to_be, 4, u32, tw, self)
    }

    // i8
    fn write_to_i8(&mut self, tw: i8) -> Result<()> {
        let temp = unsafe { transmute::<i8, [u8; 1]>(tw) };
        self.write_all(&temp)?;
        Ok(())
    }

    // i16
    fn write_le_to_i16(&mut self, tw: i16) -> Result<()> {
        write_auto!(to_le, 2, i16, tw, self)
    }

    fn write_be_to_i16(&mut self, tw: i16) -> Result<()> {
        write_auto!(to_be, 2, i16, tw, self)
    }

    // i32
    fn write_le_to_i32(&mut self, tw: i32) -> Result<()> {
        write_auto!(to_le, 4, i32, tw, self)
    }

    fn write_be_to_i32(&mut self, tw: i32) -> Result<()> {
        write_auto!(to_be, 4, i32, tw, self)
    }

    // f32
    fn write_le_to_f32(&mut self, tw: f32) -> Result<()> {
        #[allow(unused_mut)]
        let mut temp = unsafe { transmute::<f32, [u8; 4]>(tw) };
        #[cfg(target_endian = "big")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    fn write_be_to_f32(&mut self, tw: f32) -> Result<()> {
        #[allow(unused_mut)]
        let mut temp = unsafe { transmute::<f32, [u8; 4]>(tw) };
        #[cfg(target_endian = "little")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    // f64
    fn write_le_to_f64(&mut self, tw: f64) -> Result<()> {
        #[allow(unused_mut)]
        let mut temp = unsafe { transmute::<f64, [u8; 8]>(tw) };
        #[cfg(target_endian = "big")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    fn write_be_to_f64(&mut self, tw: f64) -> Result<()> {
        #[allow(unused_mut)]
        let mut temp = unsafe { transmute::<f64, [u8; 8]>(tw) };
        #[cfg(target_endian = "little")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }
}

impl<W: Write> WriteE for W {}

pub trait MagicNumberCheck: Read {
    fn check_magic_number(&mut self, magic_number: &[u8]) -> STDResult<(), MagicNumberCheckError> {
        let mut read = vec![0u8; magic_number.len()];
        self.read_exact(&mut read)?;
        if read.as_slice() != magic_number {
            Err(MagicNumberCheckError::MagicNumber(WrongMagicNumber {
                expected: magic_number.to_vec(),
                read: read.to_vec(),
            }))
        } else {
            Ok(())
        }
    }
}

impl<R: Read> MagicNumberCheck for R {}

#[cfg(test)]
mod read_tests {
    use super::ReadE;
    use std::io::Cursor;

    #[test]
    fn u8_tests() {
        assert_eq!(120, Cursor::new([120]).read_to_u8().unwrap());
    }
    #[test]
    fn u16_tests() {
        assert_eq!(30882, Cursor::new([162, 120]).read_le_to_u16().unwrap());
        assert_eq!(30882, Cursor::new([120, 162]).read_be_to_u16().unwrap());
    }
    #[test]
    fn u32_tests() {
        assert_eq!(
            987654321u32,
            Cursor::new([177, 104, 222, 58]).read_le_to_u32().unwrap()
        );
        assert_eq!(
            987654321u32,
            Cursor::new([58, 222, 104, 177]).read_be_to_u32().unwrap()
        );
    }
    #[test]
    fn f32_tests() {
        assert_eq!(
            768f32,
            Cursor::new([0x00, 0x00, 0x40, 0x44])
                .read_le_to_f32()
                .unwrap()
        );
        assert_eq!(
            768f32,
            Cursor::new([0x44, 0x40, 0x00, 0x00])
                .read_be_to_f32()
                .unwrap()
        );
    }
    #[test]
    fn f64_tests() {
        assert_eq!(
            91024.5f64,
            Cursor::new([0x00, 0x00, 0x00, 0x00, 0x08, 0x39, 0xF6, 0x40])
                .read_le_to_f64()
                .unwrap()
        );
        assert_eq!(
            91024.5f64,
            Cursor::new([0x40, 0xF6, 0x39, 0x08, 0x00, 0x00, 0x00, 0x00])
                .read_be_to_f64()
                .unwrap()
        );
    }
}
#[cfg(test)]
mod write_tests {
    use super::WriteE;
    use std::io::Cursor;

    #[test]
    fn u8_tests() {
        let mut test_cursor: Cursor<Vec<u8>> = Cursor::new(vec![0u8; 1]);
        test_cursor.write_to_u8(120u8).unwrap();
        assert_eq!(test_cursor.into_inner(), vec![120]);
    }
    #[test]
    fn u16_tests() {
        let mut test_cursor: Cursor<Vec<u8>> = Cursor::new(vec![0u8; 2]);
        test_cursor.write_le_to_u16(30882u16).unwrap();
        assert_eq!(test_cursor.into_inner(), vec![162, 120]);
        let mut test_cursor2: Cursor<Vec<u8>> = Cursor::new(vec![0u8; 2]);
        test_cursor2.write_be_to_u16(30882u16).unwrap();
        assert_eq!(test_cursor2.into_inner(), vec![120, 162]);
    }
    #[test]
    fn f32_tests() {
        let mut test_cursor: Cursor<Vec<u8>> = Cursor::new(vec![0u8; 4]);
        test_cursor.write_le_to_f32(768f32).unwrap();
        assert_eq!(test_cursor.into_inner(), vec![0x00, 0x00, 0x40, 0x44]);
        let mut test_cursor2: Cursor<Vec<u8>> = Cursor::new(vec![0u8; 4]);
        test_cursor2.write_be_to_f32(768f32).unwrap();
        assert_eq!(test_cursor2.into_inner(), vec![0x44, 0x40, 0x00, 0x00]);
    }
    #[test]
    fn f64_tests() {
        let mut test_cursor: Cursor<Vec<u8>> = Cursor::new(vec![0u8; 8]);
        test_cursor.write_le_to_f64(91024.5f64).unwrap();
        assert_eq!(
            test_cursor.into_inner(),
            vec![0x00, 0x00, 0x00, 0x00, 0x08, 0x39, 0xF6, 0x40]
        );
        let mut test_cursor2: Cursor<Vec<u8>> = Cursor::new(vec![0u8; 8]);
        test_cursor2.write_be_to_f64(91024.5f64).unwrap();
        assert_eq!(
            test_cursor2.into_inner(),
            vec![0x40, 0xF6, 0x39, 0x08, 0x00, 0x00, 0x00, 0x00]
        );
    }
}
#[cfg(test)]
mod magic_number_tests {
    use crate::error::MagicNumberCheckError;
    use std::io::Cursor;
    use crate::MagicNumberCheck;
    #[test]
    fn check_valid() {
        let ref mut reader = Cursor::new(vec![b'T', b'E', b'S', b'T']);
        reader
            .check_magic_number(&[b'T', b'E', b'S', b'T'])
            .unwrap();
    }

    #[test]
    fn check_invalid() {
        let ref mut reader = Cursor::new(vec![b'T', b'E', b'S', b'T']);
        match reader.check_magic_number(&[b'N', b'E', b'S', b'T']) {
            Ok(_) => panic!("Expected an error"),
            Err(MagicNumberCheckError::MagicNumber(_)) => {}
            Err(_) => panic!("Unexpected error"),
        }
    }
}
