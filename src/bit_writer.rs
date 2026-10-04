use std::io::Write;

pub struct BitWriter<W: Write> {
    writer: W,
    accumulator: u8,
    bits_in_accumulator: u8,
}

impl<W: Write> BitWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            accumulator: 0,
            bits_in_accumulator: 0,
        }
    }

    pub fn write_bits(&mut self, mut value: u16, mut count: u8) -> std::io::Result<()> {
        while count > 0 {
            let space_left = 8 - self.bits_in_accumulator;

            if count >= space_left {
                let shift = count - space_left;
                let bits_to_add = (value >> shift) as u8;

                self.accumulator |= bits_to_add;
                self.writer.write_all(&[self.accumulator])?;

                self.accumulator = 0;
                self.bits_in_accumulator = 0;
                count -= space_left;
                if count < 16 {
                    value &= (1 << count) - 1;
                }
            } else {
                self.accumulator |= (value as u8) << (space_left - count);
                self.bits_in_accumulator += count;
                count = 0;
            }
        }
        Ok(())
    }

    pub fn finish(mut self) -> std::io::Result<()> {
        if self.bits_in_accumulator > 0 {
            self.writer.write_all(&[self.accumulator])?;
        }
        self.writer.flush()?;
        Ok(())
    }
}