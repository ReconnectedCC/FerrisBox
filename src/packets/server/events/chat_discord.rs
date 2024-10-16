use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatDiscordEvent {
    /// The message contents, without Markdown formatting codes.
    pub text: String,

    /// The message contents, with its original Markdown formatting codes.
    pub raw_text: String,

    /// The message contents, serialised with formatting as Minecraft JSON text.
    pub rendered_text: serde_json::Value,

    /// The Discord snowflake ID of this message.
    pub discord_id: String,

    /// The Discord user who sent the message.
    pub discord_user: crate::models::DiscordUser,

    /// `true` if this event represents an edit to the original message.
    pub edited: bool,
}
