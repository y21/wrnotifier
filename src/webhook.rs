use std::fs;
use std::sync::LazyLock;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;

use crate::results::EngineClass;
use crate::results::WrEntry;

#[derive(Deserialize)]
pub struct LocalWebhook {
    pub url: String,
    pub include_200cc: bool,
}

pub static WEBHOOKS: LazyLock<Vec<LocalWebhook>> = LazyLock::new(|| {
    serde_json::from_str(
        &fs::read_to_string("webhooks.json").expect("Failed to read webhooks.json"),
    )
    .expect("Failed to parse webhooks.json")
});

#[derive(Serialize)]
struct Field<'a> {
    name: &'a str,
    value: &'a str,
}

#[derive(Serialize)]
struct Embed<'a> {
    title: &'a str,
    color: u32,
    fields: &'a [Field<'a>],
}

#[derive(Serialize)]
struct Message<'a> {
    embeds: &'a [Embed<'a>],
}

pub fn send_record_to_webhook(webhook: &str, record: &WrEntry) -> anyhow::Result<()> {
    info!("sending record {record:?} to {webhook}");

    ureq::post(webhook)
        .send_json(Message {
            embeds: &[Embed {
                title: "New World Record",
                color: 0xae60,
                fields: &[
                    Field {
                        name: "Track",
                        value: record.track_name,
                    },
                    Field {
                        name: "Ghost information",
                        value: &format!(
                            "Player: {}\nTime: {}\nBest lap: {}\nCharacter: {}\nVehicle: {}",
                            record.player,
                            record.finish_time_simple,
                            record.best_split_simple,
                            record.character,
                            record.vehicle
                        ),
                    },
                    Field {
                        name: "Engine class",
                        value: match record.engine_class {
                            EngineClass::E150 => "150cc",
                            EngineClass::E200 => "200cc",
                        },
                    },
                ],
            }],
        })
        .context("Failed to execute webhook")?;

    Ok(())
}
