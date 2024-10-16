use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AfkReturnEvent {
    /// The in-game player who returned from being AFK.
    pub user: crate::models::User,
}
