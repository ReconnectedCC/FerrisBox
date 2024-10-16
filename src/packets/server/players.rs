use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayersPacket {
    pub time: String,
    pub players: Vec<crate::models::User>,
}
