use crate::class_reader_error::{ClassReaderError, Result};
use cesu8::from_java_cesu8;

pub struct BufferReader<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> BufferReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BufferReader {
            buffer: data,
            position: 0,
        }
    }

    fn advance(&mut self, size: usize) -> Result<&[u8]> {
        if self.position + size > self.buffer.len() {
            Err(ClassReaderError::InvalidClassData(
                "class does not have expected length".to_string(),
            ))
        } else {
            let slice = &self.buffer[self.position..self.position + size];
            self.position += size;
            Ok(slice)
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        self.advance(std::mem::size_of::<u8>())
            .map(|bytes| u8::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.advance(std::mem::size_of::<u16>())
            .map(|bytes| u16::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        self.advance(std::mem::size_of::<u32>())
            .map(|bytes| u32::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        self.advance(std::mem::size_of::<i32>())
            .map(|bytes| i32::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        self.advance(std::mem::size_of::<i64>())
            .map(|bytes| i64::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        self.advance(std::mem::size_of::<f32>())
            .map(|bytes| f32::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        self.advance(std::mem::size_of::<f64>())
            .map(|bytes| f64::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_utf8(&mut self, len: usize) -> Result<String> {
        self.advance(len)
            .and_then(|bytes| {
                from_java_cesu8(bytes).map_err(|_| {
                    ClassReaderError::InvalidClassData("invalid utf8 data".to_string())
                })
            })
            .map(|cow_string| cow_string.into_owned())
    }

    #[allow(dead_code)]
    fn has_more_data(&self) -> bool {
        self.position < self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::BufferReader;

    #[test]
    fn buffer_works() {
        let data = vec![0x00, 0x00, 0x00, 0x42];
        let mut buffer = BufferReader::new(&data);

        assert_eq!(true, buffer.has_more_data());
        assert_eq!(0x42u32, buffer.read_u32().unwrap());
        assert_eq!(false, buffer.has_more_data());

        assert_eq!(true, buffer.read_u32().is_err());
    }
}
