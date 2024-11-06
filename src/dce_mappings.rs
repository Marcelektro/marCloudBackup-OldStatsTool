use chrono::{DateTime, Utc};
use serde::Deserialize;

/*
Mappings for the JSON schema used by https://github.com/Tyrrrz/DiscordChatExporter
*/


// #[derive(Deserialize, Debug)]
// pub struct Author {
//     pub id: String,
//     pub name: String
// }

#[derive(Deserialize, Debug)]
pub struct Embed {
    // pub title: Option<String>,
    pub description: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Message {
    // pub id: String,
    pub timestamp: DateTime<Utc>,
    pub content: String,
    // pub author: Author,
    pub embeds: Vec<Embed>
}

#[derive(Deserialize, Debug)]
pub struct Guild {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Channel {
    pub id: String,
    pub category: Option<String>,
    pub name: String
}


#[derive(Deserialize, Debug)]
pub struct Root {
    pub guild: Guild,
    pub channel: Channel,
    pub messages: Vec<Message> // TODO: avoid buffering
}
