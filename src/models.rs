use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct DatabaseWebhook {
    pub id: String,
    pub token: String,
    pub server: String,
    #[sqlx(rename = "150ccOnly")]
    pub no_200c: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexResponseBody {
    pub recent_records: Vec<Record>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub hash: String,
    pub track_name: String,
    pub track_version: Option<String>,
    pub player: String,
    pub finish_time_simple: String,
    pub best_split_simple: String,
    pub href: String,
    #[serde(rename = "200cc")]
    pub is_200cc: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct DiscordMessage<'a> {
    pub embeds: &'a [DiscordEmbed<'a>],
}

#[derive(Serialize, Debug, Clone)]
pub struct DiscordEmbed<'a> {
    pub title: &'a str,
    pub color: u32,
    pub fields: &'a [DiscordEmbedField<'a>],
}

#[derive(Serialize, Debug, Clone)]
pub struct DiscordEmbedField<'a> {
    pub name: &'a str,
    pub value: &'a str,
}
