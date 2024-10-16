use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelloPacket {
    pub guest: bool,
    pub license_owner: String,
    pub license_owner_user: serde_json::Value,
    pub capabilities: Vec<String>,
}
