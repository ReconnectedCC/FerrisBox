use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeathEvent {
    /// The death message contents, without formatting codes.
    pub text: String,

    /// The death message contents, with its original formatting codes.
    pub raw_text: String,

    /// The death message contents, serialised with formatting as Minecraft JSON text.
    pub rendered_text: serde_json::Value,

    /// The in-game player who died.
    pub user: crate::models::User,

    /// The player that killed this player.
    pub source: Option<crate::models::User>,
}
