use futures::{SinkExt, Stream, StreamExt};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub mod models;
pub mod packets;

#[derive(Debug)]
pub struct ChatboxClientInstance {
    tx: Sender<packets::ClientPacket>,
    rx: Receiver<packets::ServerPacket>,
    last_message_id: i32,
}

impl ChatboxClientInstance {
    pub async fn new(license: String, endpoint: Option<&str>) -> Self {
        let endpoint = endpoint.unwrap_or("wss://chat.reconnected.cc/v2");
        let url = format!("{}/{}", endpoint, license);

        let (stream, _) = connect_async(&url).await.expect("Failed to connect");
        let (mut sender, mut receiver) = stream.split();

        let (tx, mut rx) = mpsc::channel::<packets::ClientPacket>(64);
        let (ws_tx, ws_rx) = mpsc::channel::<packets::ServerPacket>(64);

        tokio::spawn(async move {
            while let Some(incoming) = receiver.next().await {
                let message = incoming.expect("Idk what happened man.");

                if message.is_text() {
                    let message = message.into_text().expect("Got invalid UTF8");

                    if let Ok(packet) = serde_json::from_str::<packets::ServerPacket>(&message) {
                        ws_tx
                            .send(packet)
                            .await
                            .expect("Failed to send packet to reciever.")
                    }
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

        ChatboxClientInstance {
            tx,
            rx: ws_rx,
            last_message_id: 0,
        }
    }

    pub async fn tell(&mut self, message: packets::client::tell::TellPacket) {
        let tx = &self.tx;

        let packet_type = packets::client::PacketType::Tell(message);
        let packet = packets::ClientPacket {
            packet_type,
            id: self.last_message_id,
        };

        tx.send(packet)
            .await
            .expect("Failed to send packet to reciever.");

        self.last_message_id += 1;
    }

    pub async fn say(&mut self, message: packets::client::say::SayPacket) {
        let tx = &self.tx;

        let packet_type = packets::client::PacketType::Say(message);
        let packet = packets::ClientPacket {
            packet_type,
            id: self.last_message_id,
        };

        tx.send(packet)
            .await
            .expect("Failed to send packet to reciever.");

        self.last_message_id += 1;
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
