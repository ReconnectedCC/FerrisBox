use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorPacket {
    pub error: String,
    pub message: String,
}
