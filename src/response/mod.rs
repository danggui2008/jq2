use actix_web::{http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::common::constant;

#[derive(Debug,Serialize,Deserialize)]
pub struct Response<T> {
    code: u16,
    message: String,
    data: T,
}

impl<T: Serialize> Response<T> {
    pub fn http_response(status: StatusCode, message: &str, data: T) -> HttpResponse {
        let result = Self {
            code: u16::from(status),
            message: message.into(),
            data,
        };
        HttpResponse::build(StatusCode::OK).json(result)
    }

    pub fn success_with_message(message: &str, data: T) -> result::Response {
        Ok(Self::http_response(StatusCode::OK, message, data))
    }
    pub fn success(data: T) -> result::Response {
        Ok(Self::http_response(
            StatusCode::OK,
            constant::SUCCESS_MESSAGE,
            data,
        ))
    }
}

pub mod result {
    use actix_web::HttpResponse;

    use crate::common::error::ApplicationError;

    pub type Result<T, E = ApplicationError> = std::result::Result<T, E>;

    pub type Response = Result<HttpResponse>;
}
