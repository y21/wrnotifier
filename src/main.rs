use std::fs;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Context;
use tracing::debug;
use tracing::error;

use crate::results::EngineClass;
use crate::results::Results;
use crate::webhook::WEBHOOKS;
use crate::webhook::send_record_to_webhook;

mod characters;
mod results;
mod vehicles;
mod webhook;

const HOST: &str = match option_env!("TT_JSON_URL") {
    Some(url) => url,
    None => "https://tt.chadsoft.co.uk/index.json",
};

fn process() -> anyhow::Result<()> {
    debug!("fetching tt data from {HOST}");

    let response_text = ureq::get(HOST)
        .header(
            "User-Agent",
            "wr-notifier (+https://github.com/y21/wrnotifier)",
        )
        .call()
        .context("failed to fetch TT data from server")?
        .body_mut()
        .read_to_string()
        .context("failed to read response body")?;

    let response_text = response_text
        .strip_prefix('\u{feff}')
        .unwrap_or(&response_text); // Remove BOM if present

    let json =
        serde_json::from_str::<Results>(response_text).context("failed to parse JSON response")?;

    if let Ok(last_ghost_hash) = fs::read_to_string("last.txt")
        && let last_ghost_hash = last_ghost_hash.trim()
        && let Some(last_seen_pos) = json
            .recent_records
            .iter()
            .position(|entry| entry.hash == last_ghost_hash)
    {
        let new_ghosts = &json.recent_records[..last_seen_pos];

        if !new_ghosts.is_empty() {
            debug!("found new records: {new_ghosts:?}");
        }

        for webhook in &*WEBHOOKS {
            for record in new_ghosts {
                if (record.engine_class == EngineClass::E200 && !webhook.class_200cc)
                    || (record.engine_class == EngineClass::E150 && !webhook.class_150cc)
                {
                    // Webhook is not interested in this engine class
                    continue;
                }

                send_record_to_webhook(&webhook.url, record)
                    .context("failed to send record to webhook")?;
            }
        }
    }

    fs::write("last.txt", json.recent_records[0].hash)
        .context("failed to write last ghost hash to file")?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    loop {
        if let Err(err) = process() {
            error!("an error occurred: {err:?}");
        }

        sleep(Duration::from_secs(60 * 5));
    }
}
