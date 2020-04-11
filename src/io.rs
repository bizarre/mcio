use std::io::{ Read, Write, Result };

pub trait MinecraftWrite {
    fn write_varint(&mut self, value: i32) -> Result<()>;
    fn write_string(&mut self, value: String) -> Result<()>;
    fn write_u16(&mut self, value: u16) -> Result<()>;
}

impl<W: Write> MinecraftWrite for W {
    fn write_varint(&mut self, value: i32) -> Result<()> {
        let mut value = value.to_owned();
        let mut buffer = [0; 5]; // VarInts are never longer than 5 bytes

        loop {
            let mut temp = (value & 0b01111111) as u8;

            value >>= 7;
            if (value != 0) {
                temp |= 0b10000000;
            }

            buffer[buffer.len()] = temp;

            if (value == 0) {
                break;
            }
        }

        self.write_all(&mut buffer)?;

        Ok(())
    }

    fn write_string(&mut self, value: String) -> Result<()> {
        self.write_varint(value.len() as i32)?;
        self.write_all(value.as_bytes())?;

        Ok(())
    }

    fn write_u16(&mut self, value: u16) -> Result<()> {
        Ok(())
    }
}