use std::sync::Arc;

use crate::app::App;

pub async fn get_webhooks(app: Arc<App>) -> Result<impl warp::Reply, warp::Rejection> {
    let webhooks = app.db.get_webhooks().await.unwrap();
    Ok(warp::reply::json(&webhooks))
}
