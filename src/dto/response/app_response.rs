
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::util::serializer::date_serializer;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AppResponse<T> {
    pub status: String,
    pub message: String,
    #[serde(with = "date_serializer")]
    pub timestamp: NaiveDateTime,
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AppResponseError<T> {
    pub status: String,
    pub message: String,
    #[serde(with = "date_serializer")]
    pub timestamp: NaiveDateTime,
    pub error: T,
}
