use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandEvent {
    /// The in-game player who ran the command.
    pub user: crate::models::User,

    /// The name of the command (the word immediately following the backslash/caret/pipe, excluding the backslash/caret/pipe).
    pub command: String,

    /// Array of space-separated string arguments after the command.
    pub args: Vec<String>,

    /// `true` if the command is an owner-only command (`^command`).
    pub owner_only: bool,
}
