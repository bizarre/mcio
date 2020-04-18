use std::env;

use std::io::{ Result };

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let default_server = "localhost".to_owned();
    let default_port = "25577".to_owned();

    let address = args.get(1).unwrap_or(&default_server);
    let port = args.get(2).unwrap_or(&default_port).parse::<u16>().unwrap();

    let response = mcio::ping(address, port, 315).expect("Failed to get response.");

    println!("{:?}", response);

    Ok(())
}