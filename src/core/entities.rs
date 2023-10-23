use std::path::Path;

use chrono::{DateTime, Local};

pub struct UploadedFile<I> {
    pub id: I,
    pub filename: String,
    pub mime_type: String,
    pub filepath: String,
    pub uploader_id: I,
    pub uploaded_at: DateTime<Local>,
}

pub struct UploadedFileCreate<I> {
    pub filename: String,
    pub mime_type: String,
    pub filepath: String,
    pub uploader_id: I,
}

#[derive(Debug, Clone)]
pub struct UploadedFileQuery<I> {
    pub id_eq: Option<I>,
    pub uploader_id_eq: Option<I>,
}
