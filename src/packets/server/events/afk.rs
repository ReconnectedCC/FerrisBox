use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AfkEvent {
    /// The in-game player who went AFK.
    pub user: crate::models::User,
}
