use actix_http::StatusCode;
use actix_web::{error::ResponseError, web::Json, HttpResponse};
use serde_json::json;
use sqlx::Error as SqlxError;
use thiserror::Error as TError;

pub type Result<T> = std::result::Result<Json<T>, ServerError>;

#[derive(TError, Debug)]
pub enum ServerError {
    #[error("You are unauthorized {0}")]
    Unauthorized(&'static str),
    #[error(transparent)]
    DatabaseError(#[from] SqlxError),
    #[error("{0}")]
    Conflict(&'static str),
    #[error("{0}")]
    NotFound(&'static str),
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({"message": self.to_string()}))
    }

    fn status_code(&self) -> actix_http::StatusCode {
        match *self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}
