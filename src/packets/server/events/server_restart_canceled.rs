use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerRestartCanceledEvent {
    pub restart_type: super::RestartType,
    pub restart_seconds: i32,
    pub restart_at: String,
}
