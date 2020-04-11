#![crate_name = "mcio"]

/// Minecraft protocol packets
pub mod packet;
pub mod io;

use packet::Packet;
use std::io::{ Result };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handshake_init() -> Result<()> {
        let handshake = Packet::Handshake(4, "localhost".to_owned(), 25565, 1);

        Ok(())
    }
}