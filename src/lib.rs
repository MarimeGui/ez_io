use std::error::Error;
use std::io::Read;
use std::io::Result as EResult;
use std::io::Write;
use std::mem::transmute;
use std::result::Result;

pub trait ReadE: Read {
    // u8
    fn read_to_u8(&mut self) -> EResult<u8> {
        let mut temp: [u8; 1] = [0];
        self.read_exact(&mut temp[..])?;
        Ok(temp[0])
    }

    // u16
    fn read_le_to_u16(&mut self) -> EResult<u16> {
        let mut temp: [u8; 2] = [0; 2];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 2], u16>(temp)) }
    }

    fn read_be_to_u16(&mut self) -> EResult<u16> {
        let mut temp: [u8; 2] = [0; 2];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 2], u16>(temp)) }
    }

    // u32
    fn read_le_to_u32(&mut self) -> EResult<u32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 4], u32>(temp)) }
    }

    fn read_be_to_u32(&mut self) -> EResult<u32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 4], u32>(temp)) }
    }

    // i8
    fn read_to_i8(&mut self) -> EResult<i8> {
        let mut temp: [u8; 1] = [0];
        self.read_exact(&mut temp[..])?;
        unsafe { Ok(transmute::<u8, i8>(temp[0])) }
    }

    // i16
    fn read_le_to_i16(&mut self) -> EResult<i16> {
        let mut temp: [u8; 2] = [0; 2];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 2], i16>(temp)) }
    }

    fn read_be_to_i16(&mut self) -> EResult<i16> {
        let mut temp: [u8; 2] = [0; 2];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 2], i16>(temp)) }
    }

    // i32
    fn read_le_to_i32(&mut self) -> EResult<i32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 4], i32>(temp)) }
    }

    fn read_be_to_i32(&mut self) -> EResult<i32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 4], i32>(temp)) }
    }

    // f32
    fn read_le_to_f32(&mut self) -> EResult<f32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 4], f32>(temp)) }
    }

    fn read_be_to_f32(&mut self) -> EResult<f32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 4], f32>(temp)) }
    }

    // f64
    fn read_le_to_f64(&mut self) -> EResult<f64> {
        let mut temp: [u8; 8] = [0; 8];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 8], f64>(temp)) }
    }

    fn read_be_to_f64(&mut self) -> EResult<f64> {
        let mut temp: [u8; 8] = [0; 8];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe { Ok(transmute::<[u8; 8], f64>(temp)) }
    }

    fn read_to_string_n(&mut self, length: u32) -> Result<String, Box<Error>> {
        let mut bytes = vec![0; length as usize];
        self.read_exact(&mut bytes)?;
        let out = String::from_utf8(bytes);
        if out.is_err() {
            Err(Box::new(out.unwrap_err()))
        } else {
            Ok(out.unwrap())
        }
    }
}

impl<R: Read + ?Sized> ReadE for R {}

pub trait WriteE: Write {
    // u8
    fn write_to_u8(&mut self, tw: u8) -> EResult<()> {
        self.write_all(&[tw])?;
        Ok(())
    }

    // u16
    fn write_le_to_u16(&mut self, tw: u16) -> EResult<()> {
        let mut temp = unsafe { transmute::<u16, [u8; 2]>(tw) };
        #[cfg(target_endian = "big")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    fn write_be_to_u16(&mut self, tw: u16) -> EResult<()> {
        let mut temp = unsafe { transmute::<u16, [u8; 2]>(tw) };
        #[cfg(target_endian = "little")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    // u32
    fn write_le_to_u32(&mut self, tw: u32) -> EResult<()> {
        let mut temp = unsafe { transmute::<u32, [u8; 4]>(tw) };
        #[cfg(target_endian = "big")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    fn write_be_to_u32(&mut self, tw: u32) -> EResult<()> {
        let mut temp = unsafe { transmute::<u32, [u8; 4]>(tw) };
        #[cfg(target_endian = "little")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    // i8
    fn write_to_i8(&mut self, tw: i8) -> EResult<()> {
        let mut temp = unsafe { transmute::<i8, [u8; 1]>(tw) };
        self.write_all(&temp)?;
        Ok(())
    }

    // i16
    fn write_le_to_i16(&mut self, tw: i16) -> EResult<()> {
        let mut temp = unsafe { transmute::<i16, [u8; 2]>(tw) };
        #[cfg(target_endian = "big")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    fn write_be_to_i16(&mut self, tw: i16) -> EResult<()> {
        let mut temp = unsafe { transmute::<i16, [u8; 2]>(tw) };
        #[cfg(target_endian = "little")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    // i32
    fn write_le_to_i32(&mut self, tw: i32) -> EResult<()> {
        let mut temp = unsafe { transmute::<i32, [u8; 4]>(tw) };
        #[cfg(target_endian = "big")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    // f32
    fn write_le_to_f32(&mut self, tw: f32) -> EResult<()> {
        let mut temp = unsafe { transmute::<f32, [u8; 4]>(tw) };
        #[cfg(target_endian = "big")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    fn write_be_to_f32(&mut self, tw: f32) -> EResult<()> {
        let mut temp = unsafe { transmute::<f32, [u8; 4]>(tw) };
        #[cfg(target_endian = "little")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    // f64
    fn write_le_to_f64(&mut self, tw: f64) -> EResult<()> {
        let mut temp = unsafe { transmute::<f64, [u8; 8]>(tw) };
        #[cfg(target_endian = "big")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    fn write_be_to_f64(&mut self, tw: f64) -> EResult<()> {
        let mut temp = unsafe { transmute::<f64, [u8; 8]>(tw) };
        #[cfg(target_endian = "little")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }

    fn write_be_to_i32(&mut self, tw: i32) -> EResult<()> {
        let mut temp = unsafe { transmute::<i32, [u8; 4]>(tw) };
        #[cfg(target_endian = "little")]
        temp.reverse();
        self.write_all(&temp)?;
        Ok(())
    }
}

impl<W: Write + ?Sized> WriteE for W {}

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
    fn string_tests() {
        let text_bytes = [72, 101, 108, 108, 111];
        assert_eq!(
            String::from("Hello"),
            Cursor::new(text_bytes).read_to_string_n(5).unwrap()
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
}
