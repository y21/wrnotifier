use crate::{
    constants,
    models::{
        DatabaseWebhook, DiscordEmbed, DiscordEmbedField, DiscordMessage, IndexResponseBody, Record,
    },
    util,
};
use reqwest::Client;
use std::fmt::Display;
use std::{sync::Arc, time::Duration};

use crate::app::App;

pub struct Worker {
    app: Arc<App>,
    client: Client,
    previous: Option<Vec<Record>>,
}

#[derive(Debug)]
enum RequestError {
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
}

impl std::error::Error for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(j) => j.fmt(f),
            Self::Reqwest(r) => r.fmt(f),
        }
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<serde_json::Error> for RequestError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl Worker {
    pub fn new(app: Arc<App>) -> Self {
        Self {
            app,
            client: Client::new(),
            previous: None,
        }
    }

    async fn get_recent_records(&self) -> Result<Vec<Record>, RequestError> {
        let text = self
            .client
            .get(constants::TT_API)
            .header(
                reqwest::header::USER_AGENT,
                "WR Notifier (https://github.com/y21/wrnotifier)",
            )
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?
            .replacen("\u{feff}", "", 1);

        serde_json::from_str::<IndexResponseBody>(&text)
            .map(|x| x.recent_records)
            .map_err(Into::into)
    }

    async fn execute_webhook(
        &self,
        webhook: DatabaseWebhook,
        record: &Record,
    ) -> Result<(), reqwest::Error> {
        let url = format!(
            "https://discordapp.com/api/webhooks/{}/{}",
            webhook.id, webhook.token
        );

        let data = DiscordMessage {
            embeds: &[DiscordEmbed {
                title: "New World Record",
                color: 0xae60,
                fields: &[
                    DiscordEmbedField {
                        name: "Track",
                        value: &record.track_name,
                    },
                    DiscordEmbedField {
                        name: "Ghost information",
                        value: &format!(
                            "Player: {}\nTime: {}\nBest lap: {}\nCharacter: {}\nVehicle: {}\n[Ghost download]({}{})",
                            record.player,
                            record.finish_time_simple,
                            record.best_split_simple,
                            util::driver_id_to_string(record.driver_id).unwrap_or("?"),
                            util::vehicle_id_to_string(record.vehicle_id).unwrap_or("?"),
                            constants::TT_API_BASE,
                            record.href
                        ),
                    },
                    DiscordEmbedField {
                        name: "Engine class",
                        value: if record.is_200cc { "200cc" } else { "150cc" },
                    },
                ],
            }],
        };

        self.client.post(url).json(&data).send().await.map(|_| ())
        // TODO: delete webhook if 401?
    }

    async fn handle_world_record(
        &mut self,
        record: &Record,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let webhooks = self.app.db.get_webhooks().await?;

        for webhook in webhooks {
            if webhook.no_200c != 0 && record.is_200cc {
                continue;
            }

            self.execute_webhook(webhook, &record).await?;

            // let's wait a bit so we don't hammer the discord api
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        Ok(())
    }

    async fn run_single(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let records = self.get_recent_records().await?;

        if self.previous.is_none() {
            self.previous = Some(records);
            return Ok(());
        }

        let old = self.previous.as_ref().unwrap();
        let first = old.first().zip(records.first());

        if let Some((old, new)) = first {
            if old.hash != new.hash {
                // New world record
                if let Err(e) = self.handle_world_record(&new).await {
                    println!("{:?}", e);
                }
            }
        }

        self.previous = Some(records);

        Ok(())
    }

    pub async fn run(mut self) {
        loop {
            if let Err(e) = self.run_single().await {
                println!("{:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    }
}
