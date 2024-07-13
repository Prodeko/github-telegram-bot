use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type ApiResult<T> = core::result::Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    InternalServerError,
}

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        println!("{:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled client error").into_response()
    }
}

impl core::fmt::Display for ApiError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
