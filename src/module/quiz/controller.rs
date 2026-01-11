use std::{collections::HashMap, sync::Arc};

use axum::{Extension, Json, extract::Path, http::StatusCode};
use validator::Validate;

use crate::{
    dto::response::{app_error::AppError, app_response::AppResponse},
    module::quiz::{
        repository,
        schema::{
            MAnswer, MMultipleChoice, MMultipleChoiceRequest, MQuestion, MQuestionAnswerRequest,
            MQuestionRequest, MQuestionResponse, MUser, MUserMultipleChoice,
            MUserMultipleChoiceRequest, MUserRequest,
        },
    },
    state::AppState,
};

/**
 * # CLIENT
 * user create account => GET find_all_user_by_username then POST insert_user
 * load question and multiple choice => GET find_all_question then GET find_all_multiple_choice_by_question_id
 * user chooce one of the answer => POST insert_user_multiple_choice
 * final score => GET find_all_question then GET find_all_multiple_choice_by_question_id then GET find_answer_by_question_id
 * # ADMIN
 * load best user => GET find_all_user_multiple_choice_by_user_id GET find_answer_by_question_id
 */

pub async fn status(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    log::info!("status: {}", _state.status);
    let status_code = StatusCode::OK;
    Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: "ok".to_string(),
        }),
    ))
}

pub async fn create_new_user(
    Extension(_state): Extension<Arc<AppState>>,
    Json(m_user_request): Json<MUserRequest>,
) -> Result<(StatusCode, Json<AppResponse<HashMap<String, String>>>), AppError> {
    log::info!("status: {}", _state.status);

    let _is_valid = match m_user_request.validate() {
        Ok(value) => value,
        Err(error) => {
            return Err(AppError::InvalidRequest(error).into());
        }
    };

    // get db connection
    let db_conn_result = _state.diesel_pool_mysql.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let new_data = MUser::from_create_request(m_user_request);
    let existing_data =
        repository::find_user_by_username(&mut db_conn, new_data.username.clone().unwrap())
            .unwrap();
    if !existing_data.is_empty() {
        return Err(AppError::DataExist);
    }

    let result = repository::insert_user(&mut db_conn, new_data.clone()).unwrap();
    if result.is_none() {
        return Err(AppError::Other(format!("save data failed")).into());
    }

    let result =
        repository::find_user_by_username(&mut db_conn, new_data.username.unwrap()).unwrap();
    let mut data = HashMap::new();
    data.insert(
        String::from("id"),
        result.get(0).unwrap().id.unwrap().to_string(),
    );
    data.insert(
        String::from("username"),
        result.get(0).unwrap().username.clone().unwrap(),
    );

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: data,
        }),
    ));
}

pub async fn find_all_question(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<MQuestionResponse>>>), AppError> {
    log::info!("status: {}", _state.status);

    // get db connection
    let db_conn_result = _state.diesel_pool_mysql.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let result_question = repository::find_all_question(&mut db_conn).unwrap();
    let result_multiple_choice = repository::find_all_multiple_choice(&mut db_conn).unwrap();

    let mut data: Vec<MQuestionResponse> = Vec::new();
    for data_question in result_question {
        let mut multiple_choice_list: Vec<MMultipleChoice> = Vec::new();
        for data_multiple_choice in result_multiple_choice.clone() {
            if data_multiple_choice.m_question_id.unwrap() == data_question.id.unwrap() {
                multiple_choice_list.push(data_multiple_choice);
            }
        }
        let m_question_response: MQuestionResponse = MQuestionResponse {
            id: Some(data_question.id.unwrap()),
            question: data_question.question,
            multiple_choice: multiple_choice_list,
        };
        data.push(m_question_response);
    }
    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: data,
        }),
    ));
}

pub async fn choose_answer(
    Extension(_state): Extension<Arc<AppState>>,
    Json(data_request): Json<MUserMultipleChoiceRequest>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    log::info!("status: {}", _state.status);

    let _is_valid = match data_request.validate() {
        Ok(value) => value,
        Err(error) => {
            return Err(AppError::InvalidRequest(error).into());
        }
    };

    // get db connection
    let db_conn_result = _state.diesel_pool_mysql.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let new_data = MUserMultipleChoice::from_create_request(data_request);

    let result = repository::insert_user_multiple_choice(&mut db_conn, new_data).unwrap();
    if result.is_none() {
        return Err(AppError::Other(format!("save data failed")).into());
    }

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: String::new(),
        }),
    ));
}

pub async fn insert_question_answer(
    Extension(_state): Extension<Arc<AppState>>,
    Json(data_request): Json<MQuestionAnswerRequest>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    log::info!("status: {}", _state.status);

    // get db connection
    let db_conn_result = _state.diesel_pool_mysql.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    // INSERT QUESTION
    let new_question = MQuestionRequest {
        question: data_request.question.clone(),
    };
    let result =
        repository::insert_question(&mut db_conn, MQuestion::from_create_request(new_question))
            .unwrap();
    if result.is_none() {
        return Err(AppError::Other(format!("save data failed")).into());
    }

    // FIND QUESTION
    let result =
        repository::find_all_question_by_question(&mut db_conn, data_request.question.unwrap())
            .unwrap();
    if result.is_empty() {
        return Err(AppError::Other(format!("load data failed")).into());
    }

    // INSERT MULTIPLE CHOICE
    let question_id = result.get(0).unwrap().id;
    let mut multiple_choice_list: Vec<MMultipleChoice> = Vec::new();
    for choice in data_request.multiple_choice {
        let multiple_choice_request: MMultipleChoiceRequest = MMultipleChoiceRequest {
            multiple_choice: Some(choice),
            m_question_id: question_id.clone(),
        };
        multiple_choice_list.push(MMultipleChoice::from_create_request(
            multiple_choice_request,
        ));
    }
    let result = repository::insert_multiple_choice(&mut db_conn, multiple_choice_list).unwrap();
    if result.is_none() {
        return Err(AppError::Other(format!("save data failed")).into());
    }

    // INSERT CORECT ANSWER
    let result =
        repository::find_all_multiple_choice_by_question_id(&mut db_conn, question_id.unwrap())
            .unwrap();
    let mut multiple_choice_id: i64 = 0;
    for data in result {
        if data.m_question_id.unwrap() == question_id.unwrap() {
            multiple_choice_id = data.id.unwrap();
            break;
        }
    }
    let result = repository::insert_answer(
        &mut db_conn,
        MAnswer {
            id: None,
            answer: data_request.correct_answer,
            multiple_choice_id: Some(multiple_choice_id),
            m_question_id: question_id,
        },
    )
    .unwrap();
    if result.is_none() {
        return Err(AppError::Other(format!("save data failed")).into());
    }

    // RETURN
    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: String::new(),
        }),
    ));
}

pub async fn get_result(
    Path(user_id): Path<i64>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<HashMap<String, i32>>>), AppError> {
    log::info!("status: {}", _state.status);

    // get db connection
    let db_conn_result = _state.diesel_pool_mysql.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let result_user_multiple_choice =
        repository::find_user_multiple_choice_by_user_id(&mut db_conn, user_id).unwrap();

    let mut correct_answer = 0;
    let total_answered = result_user_multiple_choice.len();
    for data_user_multiple_choice in result_user_multiple_choice {
        let result_answer = repository::find_answer_by_question_id_and_multiple_choice_id(
            &mut db_conn,
            data_user_multiple_choice.m_question_id.clone().unwrap(),
            data_user_multiple_choice
                .m_multiple_choice_id
                .clone()
                .unwrap(),
        )
        .unwrap();
        if !result_answer.is_empty() {
            correct_answer += 1;
        }
    }

    let mut data: HashMap<String, i32> = HashMap::new();
    data.insert("correct_answer".to_string(), correct_answer);
    data.insert(
        "total_answered".to_string(),
        total_answered.try_into().unwrap(),
    );
    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: data,
        }),
    ));
}

pub async fn get_all_result(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<HashMap<String, String>>>>), AppError> {
    log::info!("status: {}", _state.status);

    // get db connection
    let db_conn_result = _state.diesel_pool_mysql.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let mut data_result: Vec<HashMap<String, String>> = Vec::new();
    let result_user = repository::find_all_user(&mut db_conn).unwrap();
    for user in result_user {
        let result_user_multiple_choice =
            repository::find_user_multiple_choice_by_user_id(&mut db_conn, user.id.unwrap())
                .unwrap();

        let mut correct_answer = 0;
        let total_answered = result_user_multiple_choice.len();
        let mut updated_on: i64 = 0;
        for data_user_multiple_choice in result_user_multiple_choice {
            let result_answer = repository::find_answer_by_question_id_and_multiple_choice_id(
                &mut db_conn,
                data_user_multiple_choice.m_question_id.clone().unwrap(),
                data_user_multiple_choice
                    .m_multiple_choice_id
                    .clone()
                    .unwrap(),
            )
            .unwrap();
            if !result_answer.is_empty() {
                correct_answer += 1;
            }

            let user_multiple_choice_created_on = data_user_multiple_choice.created_on.unwrap();
            if user_multiple_choice_created_on > updated_on {
                updated_on = user_multiple_choice_created_on;
            }
        }

        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("correct_answer".to_string(), correct_answer.to_string());
        data.insert("total_answered".to_string(), total_answered.to_string());
        data.insert("username".to_string(), user.username.unwrap());
        data.insert("updated_on".to_string(), updated_on.to_string());
        data_result.push(data);
    }

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_str().to_string(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: data_result,
        }),
    ));
}
