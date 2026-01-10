use axum::{routing::{get, post}, Router};

use crate::module::{ m_biodata::controller::{create, delete_by_id, find_all, find_by_id, update, find_page}};


pub fn new() -> Router {
    Router::new()
    .route("/list", get(find_all))
    .route("/pagination", get(find_page))
    .route("/", post(create).put(update))
    .route("/{biodata_id}", get(find_by_id).delete(delete_by_id))
}