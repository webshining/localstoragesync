use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: RecordId,
    email: String,
    password: String,
}
