use ferrisbox::{packets::client::tell::TellPacket, ChatboxClientInstance};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let license = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("this program requires at least one argument"));

    let mut client = ChatboxClientInstance::new(license, None).await;

    client
        .tell(TellPacket {
            user: "g6ys".to_owned(),
            text: "Hello, World!".to_owned(),
            name: None,
            mode: None,
        })
        .await;

    while let Some(server_packet) = client.next().await {
        println!("{:?}", server_packet);
    }

    Ok(())
}
