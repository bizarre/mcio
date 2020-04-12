use crate::io::{ MinecraftWrite };
use std::io::{ Result, Write, Read };

/// Represents a minecraft protocol packet
pub trait Packet {
    fn get_id(&self) -> i32;
}

/// Represents a packet originating from the client (mcio) and bound for the server
pub trait Out : Packet {
    fn write<W: MinecraftWrite>(self, buffer: &mut W) -> Result<()>;
}

/// Represents a packet originating from the server and bound for the client (mcio)
pub trait In : Packet {
    
}


pub struct Handshake {
    version: i32,
    address: String,
    port: u16
}

impl Handshake {
    pub fn new<A: Into<String>>(version: i32, address: A, port: u16) -> Handshake {
        Handshake {
            version: version,
            address: address.into(),
            port: port
        }
    }
}

impl Packet for Handshake {
    fn get_id(&self) -> i32 { 0x00 }
}

impl Out for Handshake {
    fn write<W: MinecraftWrite>(self, buffer: &mut W) -> Result<()> {
        buffer.write_varint(self.version)?;
        buffer.write_string(self.address)?;
        buffer.write_u16(self.port)?;
        buffer.write_varint(1)?;

        Ok(())
    }
}


pub struct Request;
impl Request {
    pub fn new() -> Request { Request { } }
}

impl Packet for Request {
    fn get_id(&self) -> i32 { 0x00 }
}

impl Out for Request {
    fn write<W: MinecraftWrite>(self, _: &mut W) -> Result<()> { Ok(()) }
}


pub struct Ping;
impl Ping {
    pub fn new() -> Ping { Ping { } }
}

impl Packet for Ping {
    fn get_id(&self) -> i32 { 0x01 }
}

impl Out for Ping {
    fn write<W: MinecraftWrite>(self, buffer: &mut W) -> Result<()> {
        buffer.write_long(-1)?;
        Ok(()) 
    }
}
