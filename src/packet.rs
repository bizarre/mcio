use crate::io::{ MinecraftWrite };
use std::io::{ Result };

/// A minecraft protocol packet
pub enum Packet {
    /// The handshake packet as described in <https://wiki.vg/Protocol#Handshaking>
    /// 
    /// # Arguments (in order)
    ///
    /// * `i32`      (version) Protocol version
    /// * `String`   (address) Hostname or IP, e.g. localhost or 127.0.0.1, that was used to connect
    /// * `u16`      (port) Default for a server is 25565. The Notchian server does not use this information
    /// * `i32`      (state) 1 for status, 2 for login
    Handshake(i32, String, u16),

    /// The request packet as described in <https://wiki.vg/Server_List_Ping#Request>
    Request,

    /// The ping packet as described in <https://wiki.vg/Server_List_Ping#Ping>
    Ping
}

impl Packet {
    pub fn get_id(&self) -> i32 {
        match self {
            &Packet::Handshake(_, _, _) => 0x00,
            &Packet::Request => 0x00,
            &Packet::Ping => 0x01
        }
    }

    pub fn write<W: MinecraftWrite>(self, buffer: &mut W) -> Result<()> {
        match self {
            Packet::Handshake(version, address, port) => {
                buffer.write_varint(version)?;
                buffer.write_string(address)?;
                buffer.write_u16(port)?;
                buffer.write_varint(1)?;

                Ok(())
            },

            Packet::Request => Ok(()),

            Packet::Ping => {
                buffer.write_long(-1)?;

                Ok(())
            }
        }
    }
}