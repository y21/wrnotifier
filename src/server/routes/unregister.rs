use serde::Deserialize;
use std::sync::Arc;

use crate::app::App;

#[derive(Deserialize)]
pub struct UnregisterWebhookQuery {
    pub server: String,
}

pub async fn unregister_webhook(
    app: Arc<App>,
    id: String,
    token: String,
    server: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    app.db.unregister(&id, &token, &server).await.unwrap();
    Ok(warp::reply())
}
