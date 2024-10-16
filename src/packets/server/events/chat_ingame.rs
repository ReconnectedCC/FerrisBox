use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatIngameEvent {
    /// The message contents, without formatting codes.
    pub text: String,
    /// The message contents, with its original formatting codes.
    pub raw_text: String,

    /// The message contents, serialised with formatting as Minecraft JSON text.
    pub rendered_text: serde_json::Value, // TODO: Model JSON text

    /// The in-game player who sent the message.
    pub user: crate::models::User,
}
