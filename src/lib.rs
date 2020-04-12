#![crate_name = "mcio"]
#![feature(external_doc)]
#![doc(include = "../README.md")]

/// Minecraft packets
pub mod packet;

/// Internal IO utilities
pub mod io;

use std::net::{ToSocketAddrs, TcpStream};
use io::{ MinecraftWrite, MinecraftRead };
use std::time::Duration;

pub fn ping<A: Into<String>>(address: A, port: u16, protocol_version: i32) -> Option<packet::Response> {
    let address = address.into();

    if let Ok(mut stream) = TcpStream::connect_timeout(
            &format!("{}:{}", address, port).to_socket_addrs().unwrap().next().unwrap(),
            Duration::new(3, 0)) {

        stream.write_packet(packet::Handshake::new(protocol_version, address, port)).unwrap();
        stream.write_packet(packet::Request::new()).unwrap();
        stream.write_packet(packet::Ping::new()).unwrap();
    
        stream.receive::<packet::Response>().unwrap()
    } else {
        None
    }
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