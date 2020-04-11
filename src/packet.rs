use crate::io::MinecraftWrite;
use std::io::{ Result };

/// A serverbound minecraft protocol packet
pub enum Packet {
    /// The handshake packet as described in <https://wiki.vg/Protocol#Handshaking>
    /// 
    /// # Arguments (in order)
    ///
    /// * `i32`      (version) Protocol version
    /// * `String`   (address) Hostname or IP, e.g. localhost or 127.0.0.1, that was used to connect
    /// * `u16`      (port) Default for a server is 25565. The Notchian server does not use this information
    /// * `i32`      (state) 1 for status, 2 for login
    Handshake(i32, String, u16, i32)
}

impl Packet {
    pub fn id(self) -> i32 {
        match self {
            Packet::Handshake(_, _, _, _) => 0
        }
    }

    pub fn write<W: MinecraftWrite>(self, buffer: &mut W) -> Result<()> {
        match self {
            Packet::Handshake(version, address, port, state) => {
                buffer.write_varint(version)?;
                buffer.write_string(address)?;
                buffer.write_u16(port)?;
                buffer.write_varint(state)?;

                Ok(())
            }
        }
    }
}