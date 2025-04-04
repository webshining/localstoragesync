use axum::{http::StatusCode, response::IntoResponse, response::Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    SurrealError(#[from] surrealdb::Error),
    #[error("Resource not found")]
    NotFound,
    #[error("Route not found")]
    RouteNotFound,
    #[error("Conflict")]
    Conflict,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let message = self.to_string();

        let status_code = match self {
            Self::SurrealError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::RouteNotFound => StatusCode::NOT_FOUND,
            Self::Conflict => StatusCode::CONFLICT,
        };

        (status_code, message).into_response()
    }
}
