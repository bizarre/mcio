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

    println!("{}", response.json);
    /* response.json (pretty):
        {"version":{
            "name":"Requires MC 1.8-1.15",
            "protocol":315
        },
        "players":{
            "max":85000,
            "online":62822,
            "sample":[]
        },
        "description":"§aHypixel Network  §c[1.8-1.15]\n§b§lEASTER EVENT §7- §6§lTRIPLE COINS + EXP",
        "favicon":"data:image/png;base64..."}
    */
}
```
