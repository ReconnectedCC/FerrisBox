use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

use futures::{SinkExt, Stream, StreamExt};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub mod models;
pub mod packets;

pub struct ChatboxEventLoop {
    rx: Receiver<packets::ServerPacket>,
}

#[derive(Debug)]
pub struct ChatboxClientInstance {
    tx: Sender<packets::ClientPacket>,
    last_message_id: AtomicI32,
}

impl ChatboxClientInstance {
    pub async fn new(license: String, endpoint: Option<&str>) -> (ChatboxClientInstance, ChatboxEventLoop) {
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
                        if let Err(e) = ws_tx.send(packet).await {
                            tracing::error!("Failed to send packet to reciever: {}", e);
                            break;
                        }
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

        (ChatboxClientInstance {
            tx,
            last_message_id: AtomicI32::new(0),
        }, ChatboxEventLoop {
            rx: ws_rx,
        })
    }

    pub async fn tell(&self, message: packets::client::tell::TellPacket) {
        let tx = &self.tx;

        let packet_type = packets::client::PacketType::Tell(message);
        let packet = packets::ClientPacket {
            packet_type,
            id: self.last_message_id.fetch_add(1, Ordering::Relaxed),
        };

        tx.send(packet)
            .await
            .expect("Failed to send packet to reciever.");
    }

    pub async fn say(&self, message: packets::client::say::SayPacket) {
        let tx = &self.tx;

        let packet_type = packets::client::PacketType::Say(message);
        let packet = packets::ClientPacket {
            packet_type,
            id: self.last_message_id.fetch_add(1, Ordering::Relaxed),
        };

        tx.send(packet)
            .await
            .expect("Failed to send packet to reciever.");
    }
}

impl Stream for ChatboxEventLoop {
    type Item = packets::ServerPacket;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let message = self.rx.poll_recv(cx); // This seems oddly simple...

        message
    }
}
