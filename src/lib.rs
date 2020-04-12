#![crate_name = "mcio"]

/// Minecraft packets
pub mod packet;
pub mod io;

use std::net::TcpStream;
use io::{ MinecraftWrite, MinecraftRead };

pub fn ping<A: Into<String>>(address: A, port: u16, protocol_version: i32) -> Option<packet::Response> {
    let address = address.into();

    let mut stream = TcpStream::connect(format!("{}:{}", address, port)).unwrap();

    stream.write_packet(packet::Handshake::new(protocol_version, address, port)).unwrap();
    stream.write_packet(packet::Request::new()).unwrap();
    stream.write_packet(packet::Ping::new()).unwrap();

    stream.receive::<packet::Response>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{ Result };
    use std::net::TcpStream;
    use io::MinecraftWrite;

    #[test]
    fn handshake() -> Result<()> {
        let mut stream = TcpStream::connect("localhost:25577")?;
        let packet = packet::Handshake::new(335, "localhost", 25577);

        stream.write_packet(packet)?;
        stream.write_packet(packet::Request::new())?;
        stream.write_packet(packet::Ping::new())?;

        Ok(())
    }
}