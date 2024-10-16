use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEvent {
    pub text: String,
    pub raw_text: String,
    pub rendered_text: serde_json::Value,
    pub user: crate::models::User,
    pub name: String,
    pub raw_name: String,
}
