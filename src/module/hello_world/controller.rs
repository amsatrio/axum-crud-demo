use std::path::PathBuf;

use axum::{
    Json,
    body::Bytes,
    extract::{Multipart, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{Datelike, Local};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    dto::response::{app_error::AppError, app_response::AppResponse},
    module::hello_world::schema::{ErrorType, Params, ParamsFile, Payloads},
};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn response_body() -> (StatusCode, Json<AppResponse<String>>) {
    let status_code = StatusCode::OK;
    (
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: "some data".to_string(),
        }),
    )
}

pub async fn path_param(Path(name): Path<String>) -> (StatusCode, Json<AppResponse<String>>) {
    let status_code = StatusCode::OK;
    (
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: name,
        }),
    )
}

pub async fn query_param(Query(param): Query<Params>) -> (StatusCode, Json<AppResponse<Params>>) {
    let status_code = StatusCode::OK;
    (
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: param,
        }),
    )
}

pub async fn payload(Json(payload): Json<Payloads>) -> (StatusCode, Json<AppResponse<Payloads>>) {
    let status_code = StatusCode::OK;
    (
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: payload,
        }),
    )
}

pub async fn logger() -> (StatusCode, Json<AppResponse<String>>) {
    log::info!("tes");
    let status_code = StatusCode::OK;
    (
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: "".to_string(),
        }),
    )
}

pub async fn error(
    Query(query): Query<ErrorType>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    if query.code == 404 {
        return Err(AppError::NotFound);
    }
    if query.code == 500 {
        return Err(AppError::InternalServerError);
    }
    let status_code = StatusCode::OK;
    Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: "".to_string(),
        }),
    ))
}

pub async fn upload(
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    let mut data: Bytes = <Bytes>::new();
    let mut file_name: String = "".to_string();
    let mut file_type: String = "".to_string();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        if name == "file".to_string() {
            file_name = field.file_name().unwrap().to_string();
            file_type = field.content_type().unwrap_or("").to_string();
            data = field.bytes().await.unwrap();

            let category = match file_type.as_str() {
                // Image types
                "image/jpeg" | "image/png" | "image/gif" => "image",
                // Audio types
                "audio/mpeg" | "audio/wav" | "audio/ogg" => "audio",
                // Video types
                "video/mp4" | "video/x-msvideo" | "video/x-flv" => "video",
                // Document types
                "application/pdf"
                | "application/msword"
                | "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                    "document"
                }
                // Default case for unknown types
                _ => "unknown",
            };
            file_type = category.to_string();

            continue;
        }
        if name == "token".to_string() {
            let token = field.text().await.unwrap();
            log::info!("Token received: {}", token);
            continue;
        }

        // document, image, audio, video, other
        if name == "file_type".to_string() {
            file_type = field.text().await.unwrap();
            log::info!("File Type: {}", file_type);
            continue;
        }
    }

    let today = Local::now();
    let date_string = format!(
        "data/{}/{}/{:02}/{:02}",
        file_type,
        today.year(),
        today.month(),
        today.day()
    );

    let file_path = format!("{}/{}", date_string, file_name);

    // check existing file
    let result_file_exist = std::fs::exists(file_path.clone());
    if result_file_exist.is_err() {
        return Err(AppError::InternalServerError);
    }
    if result_file_exist.unwrap() {
        log::info!("file exist");
        return Err(AppError::InternalServerError);
    }

    let status_create_dir = std::fs::create_dir_all(date_string);
    if status_create_dir.is_err() {
        log::info!("create dir failed");
        return Err(AppError::InternalServerError);
    }

    // create file
    let file = File::create(&file_path).await.map_err(|e| {
        log::error!("Failed to create file: {}", e);
    });
    if file.is_err() {
        return Err(AppError::InternalServerError);
    }

    // save data to file
    let status_write_data = file.unwrap().write_all(&data).await.map_err(|e| {
        log::error!("Failed to write to file: {}", e);
    });
    if status_write_data.is_err() {
        return Err(AppError::InternalServerError);
    }

    let status_code = StatusCode::OK;
    Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: "".to_string(),
        }),
    ))
}

pub async fn download(param_file: Query<ParamsFile>) -> impl IntoResponse {
    let param: ParamsFile = param_file.0;
    let file_path = PathBuf::from(param.file_path);

    let response = match File::open(&file_path).await {
        Ok(mut file) => {
            let mut contents = Vec::new();
            if let Err(_) = file.read_to_end(&mut contents).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(AppResponse {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_str().to_string(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        data: "failed to read the file".to_string(),
                    }),
                )
                    .into_response();
            }

            let response = axum::http::Response::builder()
                .header(
                    "Content-Disposition",
                    format!("attachment; filename=\"{}\"", param.file_name),
                )
                .header("Content-Type", "application/octet-stream")
                .body(contents.into())
                .unwrap();

            Ok(response)
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(AppResponse {
                status: StatusCode::NOT_FOUND.as_str().to_string(),
                message: "error".to_owned(),
                timestamp: chrono::Utc::now().naive_utc(),
                data: "file not found".to_string(),
            }),
        )
            .into_response()),
    };

    return response.unwrap();
}
