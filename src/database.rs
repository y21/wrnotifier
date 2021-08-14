use sqlx::sqlite::SqlitePoolOptions;

use crate::models;

pub struct Database {
    pool: sqlx::SqlitePool,
}

impl Database {
    pub async fn connect(path: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new().connect(path).await?;
        let this = Self { pool };
        this.create_tables().await?;
        return Ok(this);
    }

    pub async fn create_tables(&self) -> Result<(), sqlx::Error> {
        let query = r#"CREATE TABLE IF NOT EXISTS webhooks(`id` TEXT, `token` TEXT, `server` TEXT, `150ccOnly` INTEGER)"#;

        sqlx::query(query).execute(&self.pool).await.map(|_| ())
    }

    pub async fn unregister(&self, id: &str, token: &str, server: &str) -> Result<(), sqlx::Error> {
        let query = r#"DELETE FROM webhooks WHERE id = ? AND token = ? AND server = ?"#;

        sqlx::query(query)
            .bind(id)
            .bind(token)
            .bind(server)
            .execute(&self.pool)
            .await
            .map(|_| ())
    }

    pub async fn register(
        &self,
        id: &str,
        token: &str,
        server: &str,
        no_200cc: bool,
    ) -> Result<(), sqlx::Error> {
        // TODO: this could be made into a single query
        let has_webhook = self.get_webhook_in_server(server).await?.is_some();
        // TODO2: return a specific error if the webhook already exists
        if has_webhook {
            return Ok(());
        }

        self.register_raw(id, token, server, no_200cc).await
    }

    pub async fn register_raw(
        &self,
        id: &str,
        token: &str,
        server: &str,
        no_200cc: bool,
    ) -> Result<(), sqlx::Error> {
        let query = r#"INSERT INTO webhooks VALUES(?, ?, ?, ?)"#;

        sqlx::query(query)
            .bind(id)
            .bind(token)
            .bind(server)
            .bind(no_200cc as i32)
            .execute(&self.pool)
            .await
            .map(|_| ())
    }

    pub async fn get_webhooks(&self) -> Result<Vec<models::DatabaseWebhook>, sqlx::Error> {
        let query = r#"SELECT * FROM webhooks"#;

        sqlx::query_as(query).fetch_all(&self.pool).await
    }

    pub async fn get_webhook_in_server(
        &self,
        server: &str,
    ) -> Result<Option<models::DatabaseWebhook>, sqlx::Error> {
        let query = "SELECT * FROM webhooks WHERE server = ?";

        match sqlx::query_as(query)
            .bind(server)
            .fetch_one(&self.pool)
            .await
        {
            Ok(v) => Ok(Some(v)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
