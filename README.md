# FerrisBox
An asynchronous Chatbox client implementation for [ReconnectedCC](https://reconnected.cc/) written in [Rust](https://www.rust-lang.org/)

## Example usage
Start by adding the following to your `Cargo.toml`:
```toml
futures = "0.3.31"
ferrisbox = { git = "https://github.com/ReconnectedCC/FerrisBox.git" }
```

Then you can start using the library:
```rs
use ferrisbox::{packets::client::tell::TellPacket, ChatboxClientInstance};

let mut client = ChatboxClientInstance::new(license).await;

// Telling a user something
client
    .tell(TellPacket {
        user: "g6ys".to_owned(),
        text: "Hello, World!".to_owned(),
        name: None,
        mode: None,
    })
    .await;

// Receiving messages from the chatbox server
while let Some(server_packet) = client.next().await {
    println!("{:?}", server_packet);
}
```
