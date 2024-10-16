use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinEvent {
    /// The in-game player who joined.
    pub user: crate::models::User,
}
