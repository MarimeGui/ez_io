use std::io::Read;
use std::mem::transmute;

pub trait ReadE: Read {
    fn read_to_u8(&mut self) -> u8 {
        let mut temp: [u8; 1] = [0];
        self.read_exact(&mut temp[..]).expect("Failed to read");
        temp[0]
    }

    fn read_le_to_u16(&mut self) -> u16 {
        let mut temp: [u8; 2] = [0; 2];
        self.read_exact(&mut temp[..]).expect("Failed to read");
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe {
            transmute::<[u8; 2], u16>(temp)
        }
    }

    fn read_be_to_u16(&mut self) -> u16 {
        let mut temp: [u8; 2] = [0; 2];
        self.read_exact(&mut temp[..]).expect("Failed to read");
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe {
            transmute::<[u8; 2], u16>(temp)
        }
    }

    fn read_le_to_u32(&mut self) -> u32 {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..]).expect("Failed to read");
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe {
            transmute::<[u8; 4], u32>(temp)
        }
    }

    fn read_be_to_u32(&mut self) -> u32 {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..]).expect("Failed to read");
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe {
            transmute::<[u8; 4], u32>(temp)
        }
    }

    fn read_le_to_i32(&mut self) -> i32 {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..]).expect("Failed to read");
        #[cfg(target_endian = "big")]
        temp.reverse();
        unsafe {
            transmute::<[u8; 4], i32>(temp)
        }
    }

    fn read_be_to_i32(&mut self) -> i32 {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp[..]).expect("Failed to read");
        #[cfg(target_endian = "little")]
        temp.reverse();
        unsafe {
            transmute::<[u8; 4], i32>(temp)
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
        assert_eq!(120, Cursor::new([120]).read_to_u8());
    }
    #[test]
    fn u16_tests() {
        assert_eq!(30882, Cursor::new([162, 120]).read_le_to_u16());
        assert_eq!(30882, Cursor::new([120, 162]).read_be_to_u16());
    }
}
