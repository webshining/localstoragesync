use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::app::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePayload {
    tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    id: RecordId,
}

pub async fn create_handler(
    State(AppState { ref db }): State<AppState>,
    Json(req): Json<CreatePayload>,
) -> &'static str {
    db.create::<Option<Record>>("person")
        .content(req)
        .await
        .unwrap();

    "Created"
}
