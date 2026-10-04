use std::io::Read;

pub struct BitReader<R: Read> {
    reader: R,
    current_byte: u8,
    bits_remaining: u8,
}

impl<R: Read> BitReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_byte: 0,
            bits_remaining: 0,
        }
    }

    pub fn read_bit(&mut self) -> Option<u8> {
        if self.bits_remaining == 0 {
            let mut buf = [0u8; 1];
            match self.reader.read(&mut buf) {
                Ok(0) => return None,
                Ok(_) => {
                    self.current_byte = buf[0];
                    self.bits_remaining = 8;
                }
                Err(_) => return None,
            }
        }

        self.bits_remaining -= 1;

        let bit = (self.current_byte >> self.bits_remaining) & 1;
        Some(bit)
    }
}