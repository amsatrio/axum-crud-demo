use axum::{routing::{get, post}, Router};

use crate::module::hello_world::controller::{download, error, logger, path_param, payload, query_param, response_body, root, upload};

pub fn new() -> Router {
    Router::new()
    .route("/", get(root))
    .route("/path/{name}", get(path_param))
    .route("/query", get(query_param))
    .route("/response-body", get(response_body))
    .route("/payload", post(payload))
    .route("/logger", get(logger))
    .route("/error", get(error))
    .route("/file/upload", post(upload))
    .route("/file/download", get(download))
}