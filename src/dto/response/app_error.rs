use std::collections::HashMap;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use validator::ValidationErrors;

use crate::dto::response::app_response::AppResponseError;

#[derive(Debug)]
pub enum AppError {
    InvalidRequest(ValidationErrors),
    DataExist,
    NotFound,
    InternalServerError,
    Other(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DataExist => {
                let status_code = StatusCode::BAD_REQUEST;
                (
                    status_code,
                    Json(AppResponseError {
                        status: status_code.as_str().to_string(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        error: "resource exist".to_string(),
                    }),
                )
                    .into_response()
            }
            AppError::NotFound => {
                let status_code = StatusCode::NOT_FOUND;
                (
                    status_code,
                    Json(AppResponseError {
                        status: status_code.as_str().to_string(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        error: "resource not found".to_string(),
                    }),
                )
                    .into_response()
            }
            AppError::InternalServerError => {
                let status_code = StatusCode::INTERNAL_SERVER_ERROR;
                (
                    status_code,
                    Json(AppResponseError {
                        status: status_code.as_str().to_string(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        error: "internal server error".to_string(),
                    }),
                )
                    .into_response()
            }
            AppError::Other(message) => {
                let status_code = StatusCode::INTERNAL_SERVER_ERROR;
                (
                    status_code,
                    Json(AppResponseError {
                        status: status_code.as_str().to_string(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        error: message,
                    }),
                )
                    .into_response()
            }
            AppError::InvalidRequest(validation_errors) => {
                let status_code = StatusCode::BAD_REQUEST;
                (
                    status_code,
                    Json(AppResponseError {
                        status: status_code.as_str().to_string(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        error: parse_validation_error_message(&format!("{validation_errors}"))
                    }),
                )
                    .into_response()
            }
        }
    }
}

fn parse_validation_error_message(error_message: &str) -> HashMap<String, String> {
    let mut error_map = HashMap::new();

    for line in error_message.lines() {
        let mut parts = line.split(": ");
        let field = parts.next().unwrap().to_string();
        let message = parts.next().unwrap().to_string();
        error_map.insert(field, message);
    }

    error_map
}
