use redis::AsyncTypedCommands;
use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};

use crate::{
    dto::response::{app_error::AppError, app_response::AppResponse},
    state::AppState,
};

pub async fn publish(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    let client = redis::Client::open("redis://:8cffbaaa5bd144ab1939361a03103eed7d1af6fd9e2052b1ef73502745a3dfcc@127.0.0.1:6379/")
        .map_err(|error| AppError::Other(format!("open redis failed: {}", error)))?;
    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|error| AppError::Other(format!("get async failed: {}", error)))?;

    // Publish a message to the 'notifications' channel
    let message = "Hello from Rust!";
    conn.publish("notifications", message)
        .await
        .map_err(|error| AppError::Other(format!("publish failed: {}", error)))?;

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
