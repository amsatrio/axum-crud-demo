// @generated automatically by Diesel CLI.

diesel::table! {
    m_answer (id) {
        id -> Nullable<Bigint>,
        #[max_length = 255]
        answer -> Nullable<Varchar>,
        multiple_choice_id -> Nullable<Bigint>,
        m_question_id -> Nullable<Bigint>,
    }
}

diesel::table! {
    m_multiple_choice (id) {
        id -> Nullable<Bigint>,
        #[max_length = 255]
        multiple_choice -> Nullable<Varchar>,
        m_question_id -> Nullable<Bigint>,
    }
}

diesel::table! {
    m_question (id) {
        id -> Nullable<Bigint>,
        #[max_length = 255]
        question -> Nullable<Varchar>,
    }
}

diesel::table! {
    m_user (id) {
        id -> Nullable<Bigint>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
    }
}

diesel::table! {
    m_user_multiple_choice (id) {
        id -> Nullable<Bigint>,
        m_user_id -> Nullable<Bigint>,
        m_question_id -> Nullable<Bigint>,
        m_multiple_choice_id -> Nullable<Bigint>,
        created_on -> Nullable<Bigint>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    m_answer,
    m_multiple_choice,
    m_question,
    m_user,
    m_user_multiple_choice,
);
