use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldChangeEvent {
    /// The in-game player who changed worlds.
    pub user: crate::models::User,

    /// The identifier string of the world the player has moved from.
    pub origin: String,

    /// The identifier string of the world the player is now in.
    pub destination: String,
}
