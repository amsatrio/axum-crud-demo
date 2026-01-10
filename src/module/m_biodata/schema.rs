use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, QueryableByName};
use diesel::Selectable;
use serde::{Deserialize, Serialize};

use validator::Validate;

use crate::diesel_schema::m_biodata;
use crate::util::serializer::{date_serializer, option_date_serializer};

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Clone,
    Queryable,
    QueryableByName,
    Insertable,
    Selectable
)]
#[diesel(table_name = m_biodata)]
pub struct MBiodata {
    pub id: i64,
    pub fullname: Option<String>,
    pub mobile_phone: Option<String>,
    pub image: Option<Vec<u8>>,
    pub image_path: Option<String>,
    pub created_by: i64,
    #[serde(with = "date_serializer")]
    pub created_on: NaiveDateTime,
    pub modified_by: Option<i64>,
    #[serde(with = "option_date_serializer")]
    pub modified_on: Option<NaiveDateTime>,
    pub deleted_by: Option<i64>,
    #[serde(with = "option_date_serializer")]
    pub deleted_on: Option<NaiveDateTime>,
    pub is_delete: bool,
}

impl MBiodata {
    pub fn new(fullname: String, mobilephone: String) -> MBiodata {
        let date_now = chrono::Utc::now().naive_utc();
        MBiodata {
            id: 0,
            fullname: Some(fullname),
            mobile_phone: Some(mobilephone),
            image: None,
            image_path: None,
            created_by: 0,
            created_on: date_now,
            modified_by: None,
            modified_on: None,
            deleted_by: None,
            deleted_on: None,
            is_delete: false,
        }
    }
    pub fn from_create_request(request: MBiodataRequest) -> MBiodata {
        let date_now = chrono::Utc::now().naive_utc();
        let is_delete = request.is_delete.unwrap_or(false);
        let mut deleted_by: Option<i64> = None;
        let mut deleted_on: Option<NaiveDateTime> = None;
        if is_delete {
            deleted_by = Some(0);
            deleted_on = Some(date_now);
        }
        MBiodata {
            id: request.id.unwrap(),
            fullname: request.fullname,
            mobile_phone: request.mobile_phone,
            image: request.image,
            image_path: request.image_path,
            created_by: 0,
            created_on: date_now,
            modified_by: None,
            modified_on: None,
            deleted_by: deleted_by,
            deleted_on: deleted_on,
            is_delete: is_delete,
        }
    }
    pub fn from_update_request(request: MBiodataRequest, existing: MBiodata) -> MBiodata {
        let date_now = chrono::Utc::now().naive_utc();
        let is_delete = request.is_delete.unwrap_or(false);
        let mut deleted_by: Option<i64> = None;
        let mut deleted_on: Option<NaiveDateTime> = None;
        if is_delete {
            deleted_by = Some(0);
            deleted_on = Some(date_now);
        }
        MBiodata {
            id: request.id.unwrap(),
            fullname: request.fullname,
            mobile_phone: request.mobile_phone,
            image: request.image,
            image_path: request.image_path,
            created_by: existing.created_by,
            created_on: existing.created_on,
            modified_by: Some(0),
            modified_on: Some(date_now),
            deleted_by: deleted_by,
            deleted_on: deleted_on,
            is_delete: is_delete,
        }
    }
}



#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MBiodataRequest {
    #[validate(
        range(min = 1, max = 999999, message = "must be between 1-999999 chars"),
        required(message = "mandatory")
    )]
    pub id: Option<i64>,
    #[validate(
        length(min = 3, message = "must be greater than 3 chars"),
        required(message = "mandatory")
    )]
    pub fullname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[validate(required(message = "mandatory"))]
    pub is_delete: Option<bool>,
}
