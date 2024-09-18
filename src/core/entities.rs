use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadedFile {
    pub id: String,
    pub origin_name: String,
    pub mime_type: String,
    pub stored_name: String,
    pub uploader_id: String,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadedFileCreate {
    pub origin_name: String,
    pub mime_type: String,
    pub stored_name: String,
    pub uploader_id: String,
    pub uploaded_at: DateTime<Utc>,
}
