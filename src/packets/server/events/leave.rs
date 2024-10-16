use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaveEvent {
    /// The in-game player who left.
    pub user: crate::models::User,
}
