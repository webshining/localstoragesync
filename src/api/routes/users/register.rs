use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use crate::{
    api::{ApiResult, error::ApiError},
    app::AppState,
    models::user::User,
};

#[derive(Debug, Deserialize)]
pub struct ResgisterPayload {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    user: User,
}

pub async fn register_handler(
    State(AppState { ref db }): State<AppState>,
    Json(req): Json<ResgisterPayload>,
) -> ApiResult<Json<RegisterResponse>> {
    let mut response = db
        .query("SELECT * FROM user WHERE email = $email LIMIT 1")
        .bind(("email", req.email))
        .await?;
    let candidate = response.take::<Option<User>>(0)?;
    if candidate.is_some() {
        return Err(ApiError::Conflict);
    }
    Err(ApiError::NotFound)
}
