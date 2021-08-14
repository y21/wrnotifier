use crate::database::Database;

pub struct App {
    pub db: Database,
}

impl App {
    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        let db = Database::connect(db_path).await?;
        Ok(Self { db })
    }
}
