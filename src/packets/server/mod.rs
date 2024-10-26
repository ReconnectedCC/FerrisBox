pub mod error;
pub mod events;
pub mod hello;
pub mod players;
pub mod success;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ServerPacket {
    pub id: i32,
    pub ok: bool,
    #[serde(rename = "type", flatten)]
    pub packet_type: PacketType,
}

// Yes I am aware this is missing the closing packet type.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum PacketType {
    Hello(hello::HelloPacket),
    Error(error::ErrorPacket),
    Players(players::PlayersPacket),
    Success(success::SuccessPacket),
    Event(events::EventPacket),
    Ping,
}
