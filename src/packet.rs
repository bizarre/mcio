use crate::io::{ MinecraftWrite, MinecraftRead };
use std::io::{ Result };

/// Represents a Minecraft packet
pub trait Packet {
    fn get_id(&self) -> i32;
}

/// Represents a packet originating from the client (mcio) and bound for the server
pub trait Out : Packet {
    fn write<W: MinecraftWrite>(self, buffer: &mut W) -> Result<()>;
}

/// Represents a packet originating from the server and bound for the client (mcio)
pub trait In : Packet {
    fn read<R: MinecraftRead>(buffer: &mut R) -> Result<Option<Self>> where Self: Sized;
}


/// The handshake packet as described in <https://wiki.vg/Protocol#Handshaking> (*)
/// 
/// # Arguments (in order)
///
/// * `version`  Protocol version
/// * `address`  Hostname or IP, e.g. localhost or 127.0.0.1, that was used to connect
/// * `port`     Minecraft server default's to 25565. Might be ignored depending on the server version.
/// 
/// (*) The protocol specification defines a state enum that is expected but not definable in this struct and instead
/// provided in the `Out` `write` implementation for this packet.
/// 
/// # Usage
/// ```rust
/// use mcio::packet;
/// 
/// // constructs a handshake packet
/// packet::Handshake::new(47, "mc.hypixel.net", 25565) 
/// ```
/// 
/// ## Note
/// The address probably has to match the peer address of the `TcpStream`. For example, you'll probably run into some issues
/// trying to send a handshake packet with the address `google.com` to `yahoo.com`.
pub struct Handshake {
    version: i32,
    address: String,
    port: u16
}

impl Handshake {
    /// Constructs a new instance of this packet with the arguments provided
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

/// The request packet as described in <https://wiki.vg/Server_List_Ping#Request>
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

/// The ping packet as described in <https://wiki.vg/Server_List_Ping#Ping>
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

pub struct Response {
    pub json: String
}

impl Packet for Response {
    fn get_id(&self) -> i32 { 0x00 }
}

impl In for Response {
    fn read<R: MinecraftRead>(buffer: &mut R) -> Result<Option<Response>> {
        let _ = buffer.read_varint()?; //todo: maybe offload this kind of logic to calling function
        let id = buffer.read_varint()?;

        assert_eq!(0x00, id);

        let size = buffer.read_varint()?;
        let mut json_buffer = vec![0; size as usize];

        buffer.read_exact(&mut json_buffer)?;

        let json = String::from_utf8(json_buffer).unwrap();

        if json.len() == 0 {
            return Ok(None);
        }

        Ok(Some(Response {
            json: json
        }))
    }
}