use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatChatboxEvent {
    /// The message contents, without formatting codes.
    pub text: String,

    /// The message contents, with its original formatting codes.
    pub raw_text: String,

    /// The message contents, serialised with formatting as Minecraft JSON text.
    pub rendered_text: serde_json::Value,

    /// The owner of the chatbox that sent the message.
    pub user: crate::models::User,

    /// The name of the chatbox, without formatting codes.
    pub name: String,

    /// The name of the chatbox, with its original formatting codes.
    pub raw_name: String,
}
