#![crate_name = "mcio"]

/// Minecraft protocol packets
pub mod packet;
pub mod io;

use std::net::TcpStream;
use io::MinecraftWrite;
use packet::Packet;

pub fn ping<A: Into<String>>(address: A, port: u16, protocol_version: i32) {
    let address = address.into();

    let mut stream = TcpStream::connect(format!("{}:{}", address, port)).unwrap();

    stream.write_packet(Packet::Handshake(protocol_version, address, port)).unwrap();
    stream.write_packet(Packet::Request).unwrap();
    stream.write_packet(Packet::Ping).unwrap();

    
}

#[cfg(test)]
mod tests {
    use super::*;

    use packet::Packet;
    use std::io::{ Result };
    use std::net::TcpStream;
    use io::MinecraftWrite;

    #[test]
    fn handshake() -> Result<()> {
        let mut stream = TcpStream::connect("localhost:25577")?;
        let packet = Packet::Handshake(335, "localhost".to_owned(), 25577);

        stream.write_packet(packet)?;
        stream.write_packet(Packet::Request)?;
        stream.write_packet(Packet::Ping)?;

        Ok(())
    }
}