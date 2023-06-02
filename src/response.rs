use bson::doc;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    status: i32,
    response: T,
}

impl<T: Serialize> Response<T> {
    pub fn build(status: Status, response: T) -> HttpResult<T> {
        let status = status.convert();
        Ok(Json(Response { status, response }))
    }

    pub fn build_err(
        status: Status,
        response: String,
    ) -> Result<Json<Response<T>>, Json<Response<String>>> {
        let status = status.convert();
        Err(Json(Response { status, response }))
    }
}

pub type HttpResult<T> = Result<Json<Response<T>>, Json<Response<String>>>;

#[repr(i32)]
pub enum Status {
    Accepted = 200,
    BadRequest = 400,
    Forbidden = 403,
    NotFound = 404,
    Custom(i32),
}

impl Status {
    pub fn convert(&self) -> i32 {
        match self {
            Self::Accepted => 200,
            Self::BadRequest => 400,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::Custom(status) => *status,
        }
    }
}
