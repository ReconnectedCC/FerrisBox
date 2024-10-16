use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordUser {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub discriminator: String,
    pub avatar: String,
    pub roles: Vec<super::DiscordRole>,
    pub linked_user: Option<serde_json::Value>, // NOTE: This is a value because otherwise it would be infinite recursive.
}
