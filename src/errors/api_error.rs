use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::io;

pub struct ApiError(pub io::Error);

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> Self {
        ApiError(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {

        let status = match self.0.kind() {
            io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
            io::ErrorKind::AlreadyExists => StatusCode::CONFLICT,
            io::ErrorKind::InvalidInput => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.0.to_string()).into_response()
    }
}
  
