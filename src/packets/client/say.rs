use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SayPacket {
    /// The message to send.
    pub text: String,

    /// The name of the chatbox to show.
    ///
    /// If no name is specified, it will default to the username of the license owner.
    pub name: Option<String>,

    /// The formatting mode to use.
    ///
    /// You can use these formatting modes:
    /// * `markdown` - Discord-like [Markdown syntax](https://support.discord.com/hc/en-us/articles/210298617-Markdown-Text-101-Chat-Formatting-Bold-Italic-Underline). Supports URLs, but not colours.
    /// * `format` - Minecraft-like [formatting codes](https://minecraft.wiki/w/Formatting_codes) using ampersands (e.g. &e for yellow). Supports colours, but not URLs.
    /// * `minimessage` -  HTML-like [tags](https://docs.advntr.dev/minimessage/format.html) (e.g. `<yellow></yellow>` for yellow). Supports colours and hover events.
    pub mode: Option<String>,
}
