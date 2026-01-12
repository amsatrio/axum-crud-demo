use futures_util::StreamExt;
use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};

use crate::{
    dto::response::{app_error::AppError, app_response::AppResponse},
    state::AppState,
};

pub async fn subscribe(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    let client = redis::Client::open("redis://:8cffbaaa5bd144ab1939361a03103eed7d1af6fd9e2052b1ef73502745a3dfcc@127.0.0.1:6379/")
        .map_err(|error| AppError::Other(format!("open redis failed: {}", error)))?;
    let mut pubsub_conn = client
        .get_async_pubsub()
        .await
        .map_err(|error| AppError::Other(format!("get async failed: {}", error)))?;

    pubsub_conn
        .subscribe("notifications")
        .await
        .map_err(|error| AppError::Other(format!("subscribe failed: {}", error)))?;

    println!("Waiting for messages on 'notifications'...");

    let mut pubsub_stream = pubsub_conn.on_message();

    while let Some(msg) = pubsub_stream.next().await {
        let channel = msg.get_channel_name();
        let payload: String = msg
            .get_payload()
            .map_err(|error| AppError::Other(format!("get payload failed: {}", error)))?;
        println!("Received from {}: {}", channel, payload);
    }

    log::info!("status: {}", _state.status);
    let status_code = StatusCode::OK;
    Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: None,
            error: None,
        }),
    ))
}
