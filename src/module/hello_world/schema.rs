use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Params {
    name: String,
    age: Option<u32>,
}


#[derive(Deserialize, Serialize)]
pub struct Payloads {
    name: String,
    age: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorType {
    pub(crate) code: i32, // 200, 400, 404, 500
}

#[derive(Deserialize)]
pub struct ParamsFile {
    pub(crate) file_path: String,
    pub(crate) file_name: String
}