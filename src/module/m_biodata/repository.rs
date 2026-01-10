use diesel::{dsl::insert_into, prelude::*, sql_query, update};

use crate::{
    diesel_schema::m_biodata::dsl::*,
    dto::{
        database::CountResult, enumerator::filter_match_mode::FilterMatchMode, request::{filter_request::Filter, sort_request::Sort}, response::app_error::AppError
    },
    module::m_biodata::schema::MBiodata,
    util::string_manipulation,
};

pub fn find_by_id(
    conn: &mut MysqlConnection,
    biodata_id: i64,
) -> Result<Option<MBiodata>, AppError> {
    let user = m_biodata
        .filter(id.eq(biodata_id))
        .select(MBiodata::as_select())
        .first::<MBiodata>(conn)
        .optional()
        .map_err(|error| AppError::Other(format!("query failed: {}, id: {}", error, biodata_id)))?;

    Ok(user)
}

pub fn find_query_by_id(
    conn: &mut MysqlConnection,
    biodata_id: i64,
) -> Result<Option<MBiodata>, AppError> {
    let query = "SELECT * 
            FROM m_biodata 
            WHERE id = ?";

    let user: Option<MBiodata> = sql_query(query)
        .bind::<diesel::sql_types::BigInt, _>(biodata_id)
        .get_result::<MBiodata>(conn)
        .optional()
        .map_err(|error| AppError::Other(format!("query failed: {}, id: {}", error, biodata_id)))?;
    Ok(user)
}

pub fn find_all(conn: &mut MysqlConnection) -> Result<Vec<MBiodata>, AppError> {
    let query = "SELECT * 
            FROM m_biodata";

    let user: Vec<MBiodata> = sql_query(query)
        .get_results::<MBiodata>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(user)
}

pub fn delete_by_id(conn: &mut MysqlConnection, biodata_id: i64) -> Result<Option<()>, AppError> {
    let rows_affected = diesel::delete(m_biodata.filter(id.eq(biodata_id)))
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}, id: {}", error, biodata_id)))?;

    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn insert_biodata(
    conn: &mut MysqlConnection,
    biodata: MBiodata,
) -> Result<Option<()>, AppError> {
    let rows_affected = insert_into(m_biodata)
        .values(&biodata)
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn update_biodata(
    conn: &mut MysqlConnection,
    biodata: MBiodata,
) -> Result<Option<()>, AppError> {
    let rows_affected = update(m_biodata.filter(id.eq(biodata.id)))
        .set((
            modified_by.eq(biodata.modified_by),
            modified_on.eq(biodata.modified_on),
            deleted_by.eq(biodata.deleted_by),
            deleted_on.eq(biodata.deleted_on),
            is_delete.eq(biodata.is_delete),
            fullname.eq(biodata.fullname),
            mobile_phone.eq(biodata.mobile_phone),
            image.eq(biodata.image),
            image_path.eq(biodata.image_path),
        ))
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}, id: {}", error, biodata.id)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn pagination(
    conn: &mut MysqlConnection,
    page: i64,
    size: i64,
    filters: Vec<Filter>,
    sorts: Vec<Sort>,
    search: String,
) -> Result<(Vec<MBiodata>, i64), AppError> {
    // Build the query
    let mut query = "SELECT *".to_string();
    let mut query_count = "SELECT COUNT(*) AS count".to_string();
    let query_table = "FROM m_biodata".to_string();

    // Sort
    let mut query_sort = String::new();
    for sort in sorts {
        query_sort = "ORDER BY".to_owned();
        let sort_asc = if sort.desc {
            "DESC".to_string()
        } else {
            "ASC".to_string()
        };
        let sort_id = string_manipulation::cleanse_string(&sort.id);
        query_sort = format!("{} {} {}", query_sort, sort_id, sort_asc);
        break;
    }

    // Search
    let mut query_search = String::new();
    if search != String::new() {
        query_search = format!(
            "WHERE fullname LIKE '%{}%'",
            string_manipulation::cleanse_string(&search)
        );
    }

    // Filter
    let mut query_filter = "".to_string();
    for filter in filters {
        let filter_id = string_manipulation::cleanse_string(&filter.id);
        let filter_value = string_manipulation::cleanse_string(&filter.value);
        let mut filter_query_temp = "".to_string();
        match filter.match_mode {
            FilterMatchMode::CONTAINS => {
                filter_query_temp = format!("{} LIKE '%{}%'", filter_id, filter_value);
            }
            FilterMatchMode::SW => {
                filter_query_temp = format!("{} LIKE '{}%'", filter_id, filter_value);
            }
            FilterMatchMode::EW => {
                filter_query_temp = format!("{} LIKE '%{}'", filter_id, filter_value);
            }
            FilterMatchMode::BETWEEN => {}
            FilterMatchMode::EQUALS => {
                filter_query_temp = format!("{} = '{}'", filter_id, filter_value);
            }
            FilterMatchMode::NOT => {
                filter_query_temp = format!("{} <> '{}'", filter_id, filter_value);
            }
            FilterMatchMode::LT => {
                filter_query_temp = format!("{} < '{}'", filter_id, filter_value);
            }
            FilterMatchMode::GT => {
                filter_query_temp = format!("{} > '{}'", filter_id, filter_value);
            }
        };

        if query_search == "".to_owned() {
            query_search = " ".to_owned();
            query_filter = format!("WHERE {}", filter_query_temp);
        } else {
            query_filter = format!("{} AND {}", query_filter, filter_query_temp);
        }
    }

    // Pagination
    let query_pagination = format!("LIMIT {} OFFSET {}", size, size * (page));

    // Final
    query = format!(
        "{} {} {} {} {} {}",
        query, query_table, query_search, query_filter, query_sort, query_pagination
    );
    query_count = format!(
        "{} {} {} {} {}",
        query_count, query_table, query_search, query_filter, query_sort
    );
    log::info!(
        "repository > find_diesel_query_biodata_page > query: {:#?}",
        query
    );
    log::info!(
        "repository > find_diesel_query_biodata_page > query_count: {:#?}",
        query_count
    );

    let data_vec: Vec<MBiodata> = sql_query(query)
        .get_results::<MBiodata>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    let results = sql_query(query_count)
        .load::<CountResult>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok((data_vec, results[0].count))
}
