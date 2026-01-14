use redis::AsyncTypedCommands;
use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};

use crate::{
    config::environment::CONFIG, dto::response::{app_error::AppError, app_response::AppResponse}, state::AppState
};

pub async fn publish(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    let config_env = &CONFIG;
    let client = redis::Client::open(format!("redis://:{}@{}:{}/", config_env.redis_password, config_env.redis_host, config_env.redis_port))
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
