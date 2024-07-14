use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type ApiResult<T> = core::result::Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(String),
    GatewayTimeout(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        return match self {
            ApiError::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled server error").into_response()
            }
            ApiError::GatewayTimeout(_) => {
                (StatusCode::GATEWAY_TIMEOUT, "Gateway timeout").into_response()
            }
        };
    }
}

impl core::fmt::Display for ApiError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
