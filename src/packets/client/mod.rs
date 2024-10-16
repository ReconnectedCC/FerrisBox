pub mod say;
pub mod tell;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientPacket {
    #[serde(rename = "type", flatten)]
    pub packet_type: PacketType,
    pub id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum PacketType {
    Say(say::SayPacket),
    Tell(tell::TellPacket),
}
