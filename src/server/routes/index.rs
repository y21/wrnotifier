pub async fn index() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("WR Notifier")
}
