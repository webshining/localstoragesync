use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::api::ApiResult;
use crate::models::user::User;
use crate::{api::error::ApiError, app::AppState};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    user: User,
}

pub async fn login_handler(
    State(AppState { ref db }): State<AppState>,
    Json(req): Json<LoginPayload>,
) -> ApiResult<Json<LoginResponse>> {
    let mut response = db
        .query("SELECT * FROM user WHERE email = $email LIMIT 1")
        .bind(("email", req.email))
        .await?;
    let user = response
        .take::<Option<User>>(0)?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(LoginResponse { user }))
}
