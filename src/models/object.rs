use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    id: RecordId,
    filename: String,
    filetype: String,
    filesize: u64,
    filekey: String,
}
