use serde::{Deserialize, Serialize};

pub mod afk;
pub mod afk_return;
pub mod chat;
pub mod chat_chatbox;
pub mod chat_discord;
pub mod chat_ingame;
pub mod command;
pub mod death;
pub mod join;
pub mod leave;
pub mod server_restart_canceled;
pub mod server_restart_scheduled;
pub mod world_change;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RestartType {
    Automatic,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventPacket {
    #[serde(flatten)]
    pub event: EventType,

    /// The time (as ISO-8601) this player returned from being AFK according to the server.
    pub time: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "event")]
pub enum EventType {
    /// The event received when a player goes AFK in-game.
    Afk(afk::AfkEvent),

    /// The event received when a player returns from being AFK in-game.
    AfkReturn(afk_return::AfkReturnEvent),

    /// The event received when another chatbox sends a message.
    ChatChatbox(chat_chatbox::ChatChatboxEvent),

    /// The event received when a player runs a chatbox command
    /// (public backslash commands: `\command`, private owner-only caret/pipe commands: `^command`) in-game.
    ///
    /// The command capability is required to receive command events.
    Command(command::CommandEvent),

    /// The event received when a player dies in-game.
    Death(death::DeathEvent),

    /// The event received when a player posts a message in Discord.
    ChatDiscord(chat_discord::ChatDiscordEvent),

    /// The event received when a player posts a message in public chat.
    ChatIngame(chat_ingame::ChatIngameEvent),

    /// The event received when a player joins the game.
    Join(join::JoinEvent),

    /// The event received when a player leaves the game.
    Leave(leave::LeaveEvent),

    ServerRestartCanceled(server_restart_canceled::ServerRestartCanceledEvent),
    ServerRestartScheduled(server_restart_scheduled::ServerRestartScheduledEvent),

    /// The event received when a player changes worlds.
    WorldChange(world_change::WorldChangeEvent),
}
