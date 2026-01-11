use diesel::{MysqlConnection, RunQueryDsl, dsl::insert_into, sql_query};

use crate::{dto::response::app_error::AppError, module::quiz::schema::{MAnswer, MMultipleChoice, MQuestion, MUser, MUserMultipleChoice}, schema::{m_answer::dsl::m_answer, m_multiple_choice::dsl::*, m_question::dsl::*, m_user::dsl::m_user, m_user_multiple_choice::dsl::*}};


// =====================================================================================================
// MAnswer
// =====================================================================================================

pub fn find_answer_by_question_id(
    conn: &mut MysqlConnection,
    question_id: i64,
) -> Result<Vec<MAnswer>, AppError> {
    let query = "SELECT * FROM m_answer WHERE m_question_id = ?";

    let data= sql_query(query)
        .bind::<diesel::sql_types::BigInt, _>(question_id)
        .get_results::<MAnswer>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(data)
}

pub fn find_answer_by_question_id_and_multiple_choice_id(
    conn: &mut MysqlConnection,
    question_id: i64, data_multiple_choice_id: i64
) -> Result<Vec<MAnswer>, AppError> {
    let query = "SELECT * 
            FROM m_answer WHERE m_question_id = ? AND multiple_choice_id = ?";

    let data: Vec<MAnswer> = sql_query(query)
        .bind::<diesel::sql_types::BigInt, _>(question_id)
        .bind::<diesel::sql_types::BigInt, _>(data_multiple_choice_id)
        .get_results::<MAnswer>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(data)
}

pub fn insert_answer(
    conn: &mut MysqlConnection,
    data: MAnswer,
) -> Result<Option<()>, AppError> {
    let rows_affected = insert_into(m_answer)
        .values(&data)
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

// =====================================================================================================
// MUser
// =====================================================================================================


pub fn find_all_user(
    conn: &mut MysqlConnection,
) -> Result<Vec<MUser>, AppError> {
    let query = "SELECT * 
            FROM m_user";

    let data: Vec<MUser> = sql_query(query)
        .get_results::<MUser>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(data)
}

pub fn find_user_by_username(
    conn: &mut MysqlConnection,
    user_name: String,
) -> Result<Vec<MUser>, AppError> {
    let query = "SELECT * 
            FROM m_user WHERE username = ?";

    let data: Vec<MUser> = sql_query(query)
        .bind::<diesel::sql_types::VarChar, _>(user_name)
        .get_results::<MUser>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(data)
}

pub fn insert_user(conn: &mut MysqlConnection, data: MUser) -> Result<Option<()>, AppError> {
    let rows_affected = insert_into(m_user)
        .values(&data)
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}


// =====================================================================================================
// MQuestion
// =====================================================================================================

pub fn find_all_question(conn: &mut MysqlConnection) -> Result<Vec<MQuestion>, AppError> {
    let query = "SELECT * 
            FROM m_question";

    let data: Vec<MQuestion> = sql_query(query)
        .get_results::<MQuestion>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(data)
}

pub fn find_all_question_by_question(
    conn: &mut MysqlConnection,
    question_string: String,
) -> Result<Vec<MQuestion>, AppError> {
    let query = "SELECT * 
            FROM m_question where question = ?";

    let data: Vec<MQuestion> = sql_query(query)
        .bind::<diesel::sql_types::VarChar, _>(question_string)
        .get_results::<MQuestion>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(data)
}

pub fn insert_question(
    conn: &mut MysqlConnection,
    data: MQuestion,
) -> Result<Option<()>, AppError> {
    let rows_affected = insert_into(m_question)
        .values(&data)
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}


// =====================================================================================================
// MMultipleChoice
// =====================================================================================================


pub fn find_all_multiple_choice(
    conn: &mut MysqlConnection,
) -> Result<Vec<MMultipleChoice>, AppError> {
    let query = "SELECT * 
            FROM m_multiple_choice";

    let data: Vec<MMultipleChoice> = sql_query(query)
        .load::<MMultipleChoice>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(data)
}
pub fn find_all_multiple_choice_by_question_id(
    conn: &mut MysqlConnection,
    question_id: i64,
) -> Result<Vec<MMultipleChoice>, AppError> {
    let query = "SELECT * 
            FROM m_multiple_choice WHERE m_question_id = ?";

    let data: Vec<MMultipleChoice> = sql_query(query)
        .bind::<diesel::sql_types::BigInt, _>(question_id)
        .get_results::<MMultipleChoice>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(data)
}

pub fn insert_multiple_choice(
    conn: &mut MysqlConnection,
    data: Vec<MMultipleChoice>,
) -> Result<Option<()>, AppError> {
    let rows_affected = insert_into(m_multiple_choice)
        .values(&data)
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

// =====================================================================================================
// MUserMultipleChoice
// =====================================================================================================


pub fn insert_user_multiple_choice(
    conn: &mut MysqlConnection,
    data: MUserMultipleChoice,
) -> Result<Option<()>, AppError> {
    let rows_affected = insert_into(m_user_multiple_choice)
        .values(&data)
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn find_user_multiple_choice_by_user_id(
    conn: &mut MysqlConnection,
    user_id: i64,
) -> Result<Vec<MUserMultipleChoice>, AppError> {
    let query = "SELECT * 
            FROM m_user_multiple_choice 
            WHERE m_user_id = ? 
            ORDER BY created_on ASC";

    let data: Vec<MUserMultipleChoice> = sql_query(query)
        .bind::<diesel::sql_types::BigInt, _>(user_id)
        .get_results::<MUserMultipleChoice>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(data)
}


