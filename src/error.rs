use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use thiserror::Error;

/// An error returned when dealing with the blacklist.
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum ServerError {
    #[error("internal error: {0}")]
    InternalError(String),

    #[error("cannot find library with ID {0} in the server")]
    LibraryNotFound(String),
}

// Use default implementation for `error_response()` method
impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::LibraryNotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}
