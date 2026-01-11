use crate::schema::{m_answer, m_multiple_choice, m_question, m_user, m_user_multiple_choice};
use diesel::Selectable;
use diesel::prelude::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};
use validator::Validate;

// =====================================================================================================
// MMultipleChoice
// =====================================================================================================

#[derive(
    Debug, Deserialize, Serialize, Clone, Queryable, QueryableByName, Insertable, Selectable,
)]
#[diesel(table_name = m_multiple_choice)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct MMultipleChoice {
    pub id: Option<i64>,
    pub multiple_choice: Option<String>,
    pub m_question_id: Option<i64>,
}

impl MMultipleChoice {
    pub fn from_create_request(request: MMultipleChoiceRequest) -> MMultipleChoice {
        MMultipleChoice {
            id: None,
            multiple_choice: request.multiple_choice,
            m_question_id: request.m_question_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MMultipleChoiceRequest {
    pub multiple_choice: Option<String>,
    pub m_question_id: Option<i64>,
}
// =====================================================================================================
// MQuestion
// =====================================================================================================


#[derive(
    Debug, Deserialize, Serialize, Clone, Queryable, QueryableByName, Insertable, Selectable,
)]
#[diesel(table_name = m_question)]
pub struct MQuestion {
    pub id: Option<i64>,
    pub question: Option<String>,
}

impl MQuestion {
    pub fn new(question: String) -> MQuestion {
        MQuestion {
            id: None,
            question: Some(question),
        }
    }

    pub fn from_create_request(request: MQuestionRequest) -> MQuestion {
        MQuestion {
            id: None,
            question: request.question,
        }
    }

    pub fn from_update_request(request: MQuestionRequest, _existing: MQuestion) -> MQuestion {
        MQuestion {
            id: None,
            question: request.question,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MQuestionRequest {
    #[validate(length(min = 3, message = "must be greater than 3 chars"))]
    pub question: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MQuestionResponse {
    pub id: Option<i64>,
    pub question: Option<String>,
    pub multiple_choice: Vec<MMultipleChoice>,
}


// =====================================================================================================
// MUser
// =====================================================================================================


#[derive(
    Debug, Deserialize, Serialize, Clone, Queryable, QueryableByName, Insertable, Selectable,
)]
#[diesel(table_name = m_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct MUser {
    pub id: Option<i64>,
    pub username: Option<String>,
}

impl MUser {
    pub fn new(username: String) -> MUser {
        MUser {
            id: None,
            username: Some(username),
        }
    }

    pub fn from_create_request(request: MUserRequest) -> MUser {
        let mut username = String::new();
        if request.username.is_some() {
            username = request.username.unwrap();
        }
        MUser {
            id: None,
            username: Some(username),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MUserRequest {
    #[validate(length(min = 3, message = "username must be at least 3 chars"))]
    pub username: Option<String>,
}



// =====================================================================================================
// MAnswer
// =====================================================================================================

#[derive(
    Debug, Deserialize, Serialize, Clone, Queryable, QueryableByName, Insertable, Selectable,
)]
#[diesel(table_name = m_answer)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct MAnswer {
    pub id: Option<i64>,
    pub answer: Option<String>,
    pub multiple_choice_id: Option<i64>,
    pub m_question_id: Option<i64>,
}

impl MAnswer {
    pub fn new(answer: String, mc_id: i64, q_id: i64) -> MAnswer {
        MAnswer {
            id: None,
            answer: Some(answer),
            multiple_choice_id: Some(mc_id),
            m_question_id: Some(q_id),
        }
    }

    pub fn from_create_request(request: MAnswerRequest) -> MAnswer {
        MAnswer {
            id: None,
            answer: request.answer,
            multiple_choice_id: request.multiple_choice_id,
            m_question_id: request.m_question_id,
        }
    }

    pub fn from_update_request(request: MAnswerRequest, _existing: MAnswer) -> MAnswer {
        MAnswer {
            id: None,
            answer: request.answer,
            multiple_choice_id: request.multiple_choice_id,
            m_question_id: request.m_question_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MAnswerRequest {
    #[validate(length(min = 1, message = "cannot be empty"))]
    pub answer: Option<String>,
    pub multiple_choice_id: Option<i64>,
    pub m_question_id: Option<i64>,
}


// =====================================================================================================
// MUserMultipleChoice
// =====================================================================================================

#[derive(
    Debug, Deserialize, Serialize, Clone, Queryable, QueryableByName, Insertable, Selectable,
)]
#[diesel(table_name = m_user_multiple_choice)]
pub struct MUserMultipleChoice {
    pub id: Option<i64>,
    pub m_user_id: Option<i64>,
    pub m_question_id: Option<i64>,
    pub m_multiple_choice_id: Option<i64>,
    pub created_on: Option<i64>,
}

impl MUserMultipleChoice {
    pub fn from_create_request(request: MUserMultipleChoiceRequest) -> MUserMultipleChoice {
        let date_now = chrono::Utc::now().naive_utc().and_utc().timestamp();
        MUserMultipleChoice {
            id: None,
            m_user_id: request.m_user_id,
            m_question_id: request.m_question_id,
            m_multiple_choice_id: request.m_multiple_choice_id,
            created_on: Some(date_now)
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MUserMultipleChoiceRequest {
    pub m_user_id: Option<i64>,
    pub m_question_id: Option<i64>,
    pub m_multiple_choice_id: Option<i64>,
}


// =====================================================================================================
// OTHER
// =====================================================================================================


#[derive(Debug, Deserialize, Serialize)]
pub struct MQuestionAnswerRequest {
    pub question: Option<String>,
    pub multiple_choice: Vec<String>,
    pub correct_answer: Option<String>,
}