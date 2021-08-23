use std::sync::Arc;
use warp::Filter;

use crate::app::App;

use super::routes;

pub async fn handle(app: Arc<App>, port: u16, auth: Option<Box<str>>) {
    eprintln!("Running on port: {}, auth enabled: {}", port, auth.is_some());
    let auth = auth.map(|x| Box::leak(x) as &'static str);
    let app_filter = warp::any().map(move || Arc::clone(&app));

    let index = warp::path::end().and_then(routes::index);

    let auth_filter = match auth {
        Some(auth) => warp::header::exact("Authorization", auth).boxed(),
        None => warp::any().boxed(),
    };

    let register = app_filter
        .clone()
        .and(warp::path!("register" / String / String))
        .and(warp::query())
        .and(warp::post())
        .and(auth_filter.clone())
        .and_then(routes::register_webhook);

    let unregister = app_filter
        .clone()
        .and(warp::path!("unregister" / String / String))
        .and(warp::post())
        .and(warp::query().map(|query: routes::UnregisterWebhookQuery| query.server))
        .and(auth_filter.clone())
        .and_then(routes::unregister_webhook);

    let webhooks = app_filter
        .clone()
        .and(warp::path("webhooks"))
        .and(auth_filter.clone())
        .and_then(routes::get_webhooks);

    let routes = webhooks.or(unregister).or(register).or(index);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    // TODO: do we want to free the leaked auth string here?
}
