use std::env;

use mcio::packet::Packet;
use std::io::{ Result };
use std::net::TcpStream;
use mcio::io::{ MinecraftWrite };

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let default_server = "mc.hypixel.net".to_owned();
    let default_port = "25565".to_owned();

    let address = args.get(1).unwrap_or(&default_server);
    let port = args.get(2).unwrap_or(default_server);


    Ok(())
}