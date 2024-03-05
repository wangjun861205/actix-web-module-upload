use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadedFile {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub filepath: String,
    pub uploader_id: String,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadedFileCreate {
    pub filename: String,
    pub mime_type: String,
    pub filepath: String,
    pub uploader_id: String,
    pub uploaded_at: DateTime<Utc>,
}
