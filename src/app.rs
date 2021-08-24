use reqwest::Client;

use crate::database::Database;

pub struct App {
    pub client: Client,
    pub db: Database,
}

impl App {
    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        let db = Database::connect(db_path).await?;

        Ok(Self {
            client: Client::new(),
            db,
        })
    }
}
