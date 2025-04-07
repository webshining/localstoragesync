use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    id: RecordId,
    name: String,
    ip_address: String,
    device_type: String,
    last_seen: String,
    scopes: Vec<String>,
}
