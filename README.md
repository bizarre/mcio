mcio is a simple library for Rust that completes a Minecraft handshake and server list ping. It can be used to fetch a Minecraft server's:

* Version (including name and protocol number)
* Players
  * Max players
  * Online players
  * Online players sample
* MOTD
* Icon

As of right now, all calls to `mcio` are blocking.

# Example Usage
```rust
fn main() {
    let response = mcio::ping("mc.hypixel.net", 25565, 315).expect("Failed to get response.");

    println!("Players: {}/{}", response.players.online, response.players.max);
    /* Players: 62075/85000 */
}
```
