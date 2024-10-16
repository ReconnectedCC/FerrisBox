use futures::{SinkExt, Stream, StreamExt};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub mod models;
pub mod packets;

#[derive(Debug)]
pub struct ChatboxClientInstance {
    tx: Sender<packets::ClientPacket>,
    rx: Receiver<packets::ServerPacket>,
}

impl ChatboxClientInstance {
    pub async fn new(license: String) -> Self {
        let url = format!("wss://chat.reconnected.cc/v2/{}", license);
        let (stream, _) = connect_async(&url).await.expect("Failed to connect");
        let (mut sender, mut receiver) = stream.split();

        let (tx, mut rx) = mpsc::channel::<packets::ClientPacket>(64);
        let (ws_tx, ws_rx) = mpsc::channel::<packets::ServerPacket>(64);

        tokio::spawn(async move {
            while let Some(incoming) = receiver.next().await {
                let message = incoming.expect("Idk what happened man.");

                if message.is_text() {
                    let message = message.into_text().expect("Got invalid UTF8");
                    let packet: packets::ServerPacket = serde_json::from_str(&message)
                        .expect("Failed to deserialize server packet");

                    ws_tx
                        .send(packet)
                        .await
                        .expect("Failed to send packet to reciever.")
                }
            }
        });

        tokio::spawn(async move {
            while let Some(incoming) = rx.recv().await {
                let message =
                    serde_json::to_string(&incoming).expect("Failed to serialize incoming packet");
                let ws_message = Message::Text(message);

                sender
                    .send(ws_message)
                    .await
                    .expect("Failed to send ws message");
            }
        });

        ChatboxClientInstance { tx, rx: ws_rx }
    }

    pub async fn tell(&self, message: packets::client::tell::TellPacket) {
        let tx = &self.tx;

        let packet_type = packets::client::PacketType::Tell(message);
        let packet = packets::ClientPacket { packet_type, id: 0 };

        tx.send(packet)
            .await
            .expect("Failed to send packet to reciever.");
    }

    pub async fn say(&self, message: packets::client::say::SayPacket) {
        let tx = &self.tx;

        let packet_type = packets::client::PacketType::Say(message);
        let packet = packets::ClientPacket { packet_type, id: 0 };

        tx.send(packet)
            .await
            .expect("Failed to send packet to reciever.");
    }
}

impl Stream for ChatboxClientInstance {
    type Item = packets::ServerPacket;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let message = self.rx.poll_recv(cx); // This seems oddly simple...

        message
    }
}
