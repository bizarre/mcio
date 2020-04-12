use std::io::{ Read, Write, Result };
use crate::packet::{ Packet, Out, In };

pub trait MinecraftWrite {
    fn write_varint(&mut self, value: i32) -> Result<()>;
    fn write_long(&mut self, value: i64) -> Result<()>;
    fn write_string(&mut self, value: String) -> Result<()>;
    fn write_u16(&mut self, value: u16) -> Result<()>;
    fn write_packet<T: Packet + Out>(&mut self, packet: T) -> Result<()>;
}

pub trait MinecraftRead {
    fn read_varint(&mut self) -> Result<i32>;
    fn read_string(&mut self) -> Result<String>;
    fn receive<T: Packet + In>(&mut self) -> Result<()>;
}

impl<W: Write> MinecraftWrite for W {
    fn write_varint(&mut self, mut value: i32) -> Result<()> {
        let mut buffer = [0; 5]; // VarInts are never longer than 5 bytes
        let mut counter = 0;

        loop {
            let mut temp = (value & 0b01111111) as u8;

            value >>= 7;
            if value != 0 {
                temp |= 0b10000000;
            }

            buffer[counter] = temp;

            counter += 1;

            if value == 0 {
                break;
            }
        }

        self.write_all(&mut buffer[0..counter])?;

        Ok(())
    }

    fn write_long(&mut self, value: i64) -> Result<()> {
        self.write_all(&value.to_be_bytes())?;

        Ok(())
    }

    fn write_string(&mut self, value: String) -> Result<()> {
        self.write_varint(value.len() as i32)?;
        self.write_all(value.as_bytes())?;

        Ok(())
    }

    fn write_u16(&mut self, value: u16) -> Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn write_packet<T: Packet + Out>(&mut self, packet: T) -> Result<()> {
        let mut buffer = Vec::new();
        let mut payload = Vec::new();

        buffer.write_varint(packet.get_id())?;
        packet.write(&mut buffer)?;

        payload.write_varint(buffer.len() as i32)?;
        payload.write_all(&buffer)?;

        self.write_all(&payload)?;

        Ok(())
    }
}

impl<R: Read> MinecraftRead for R {
    fn read_varint(&mut self) -> Result<i32> {
        Ok(0)
    }

    fn read_string(&mut self) -> Result<String> {
        Ok("".to_owned())
    }

    fn receive<T: Packet + In>(&mut self) -> Result<()> {
        Ok(())
    }
}