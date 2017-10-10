use std::io::Read;
use std::result::Result;
use std::io::Result as EResult;
use std::error::Error;
use std::mem::transmute;

pub trait ReadE: Read {
    fn read_to_u8(&mut self) -> EResult<u8> {
        let mut temp: [u8; 1] = [0];
        self.read_exact(&mut temp[..])?;
        Ok(temp[0])
    }

    fn read_le_to_u16(&mut self) -> EResult<u16> {
        let mut temp: [u8; 2] = [0; 2];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe {
            Ok(transmute::<[u8; 2], u16>(temp))
        }
    }

    fn read_be_to_u16(&mut self) -> EResult<u16> {
        let mut temp: [u8; 2] = [0; 2];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe {
            Ok(transmute::<[u8; 2], u16>(temp))
        }
    }

    fn read_le_to_u32(&mut self) -> EResult<u32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe {
            Ok(transmute::<[u8; 4], u32>(temp))
        }
    }

    fn read_be_to_u32(&mut self) -> EResult<u32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe {
            Ok(transmute::<[u8; 4], u32>(temp))
        }
    }

    fn read_le_to_i32(&mut self) -> EResult<i32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe {
            Ok(transmute::<[u8; 4], i32>(temp))
        }
    }

    fn read_be_to_i32(&mut self) -> EResult<i32> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..])?;
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe {
            Ok(transmute::<[u8; 4], i32>(temp))
        }
    }

    fn read_to_string_n(&mut self, length: u8) -> Result<String, Box<Error>> {
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::ReadE;
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
        assert_eq!(987654321u32, Cursor::new([177, 104, 222, 58]).read_le_to_u32().unwrap());
        assert_eq!(987654321u32, Cursor::new([58, 222, 104, 177]).read_be_to_u32().unwrap());
    }
    #[test]
    fn string_tests() {
        let text_bytes = [72, 101, 108, 108, 111];
        assert_eq!(String::from("Hello"), Cursor::new(text_bytes).read_to_string_n(5).unwrap());
    }
}
