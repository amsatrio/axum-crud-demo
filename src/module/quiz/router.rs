use axum::{Router, routing::{get, post}};

use crate::module::quiz::controller::{choose_answer, create_new_user, find_all_question, find_all_question_and_answer, get_all_result, get_result, insert_question_answer, status};

pub fn new() -> Router {
    Router::new()
    .route("/status", get(status))
    .route("/create_new_user", post(create_new_user))
    .route("/find_all_question", get(find_all_question))
    .route("/find_all_question_and_answer", get(find_all_question_and_answer))
    .route("/choose_answer", post(choose_answer))
    .route("/insert_question_answer", post(insert_question_answer))
    .route("/result/{user_id}", get(get_result))
    .route("/results", get(get_all_result))
}