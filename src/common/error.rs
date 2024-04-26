use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::response::Response;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};

pub struct ApplicationError {
    pub(crate) status: StatusCode,
    pub(crate) message: String,
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code:{},msg:{}", &self.status.as_u16(), &self.message)
    }
}

impl Debug for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code:{},msg:{}", &self.status.as_u16(), &self.message)
    }
}

impl ApplicationError {
    pub fn error(message: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        Response::http_response(self.status, &self.message, ())
    }
    fn status_code(&self) -> StatusCode {
        self.status
    }
}

impl From<rbatis::Error> for ApplicationError {
    fn from(e: rbatis::Error) -> Self {
        Self::error(e.to_string())
    }
}

impl From<String> for ApplicationError {
    fn from(message: String) -> Self {
        Self::error(message)
    }
}

impl From<&str> for ApplicationError {
    fn from(message: &str) -> Self {
        Self::error(message.to_string())
    }
}

impl Error for ApplicationError {}
