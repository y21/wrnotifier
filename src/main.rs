use worker::Worker;

use crate::app::App;

mod app;
mod constants;
mod database;
mod models;
mod server;
mod worker;
mod util;

use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("WRNOTIFIER_PORT")
        .map(|v| v.parse::<u16>())
        .unwrap_or_else(|_| Ok(3000))?;

    let authorization = std::env::var("WRNOTIFIER_AUTH")
        .map(String::into_boxed_str)
        .ok();

    let app = Arc::new(App::new(constants::DATABASE_PATH).await?);

    let worker = Worker::new(Arc::clone(&app));

    let worker_handle = tokio::spawn(worker.run());
    let server_handle = tokio::spawn(server::handle(Arc::clone(&app), port, authorization));

    server_handle.await.expect("Server failed");
    worker_handle.await.expect("Worker failed");

    Ok(())
}
