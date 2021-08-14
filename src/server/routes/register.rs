use serde::Deserialize;
use std::sync::Arc;

use crate::app::App;

#[derive(Deserialize)]
pub struct RegisterWebhookQuery {
    pub server: String,
    #[serde(rename = "150ccOnly")]
    pub no_200cc: bool,
}

pub async fn register_webhook(
    app: Arc<App>,
    id: String,
    token: String,
    query: RegisterWebhookQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: regex check id

    app.db
        .register(&id, &token, &query.server, query.no_200cc)
        .await
        .unwrap();

    Ok(warp::reply())
}
