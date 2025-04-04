pub mod error;
pub mod routes;

use error::ApiError;
pub use routes::routes;

pub type ApiResult<T> = Result<T, ApiError>;
