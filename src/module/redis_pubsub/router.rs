use axum::{routing::get, Router};
use crate::module::redis_pubsub::{publisher, subscriber};

pub fn new() -> Router {
    Router::new()
    .route("/publisher", get(publisher::publish))
    .route("/subscriber", get(subscriber::subscribe))
}