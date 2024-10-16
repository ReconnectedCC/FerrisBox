use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub uuid: String,
    pub display_name: String,
    pub group: String,
    pub pronouns: Option<String>,
    pub world: String,
    pub afk: bool,
    pub alt: bool,
    pub bot: bool,
    pub linked_user: Option<super::DiscordUser>,
}
