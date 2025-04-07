use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use super::device::Device;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: RecordId,
    email: String,
    hashed_password: String,
    devices: Option<Vec<Device>>,
}
