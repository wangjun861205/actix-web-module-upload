use chrono::{DateTime, Local};

pub struct UploadedFile<I, T> {
    pub id: I,
    pub filename: String,
    pub mime_type: String,
    pub fetch_token: T,
    pub uploader_id: I,
    pub uploaded_at: DateTime<Local>,
}

pub struct UploadedFileCreate<I, T> {
    pub filename: String,
    pub mime_type: String,
    pub fetch_token: T,
    pub uploader_id: I,
}

pub struct UploadedFileQuery<I> {
    pub id_eq: Option<I>,
    pub uploader_id_eq: Option<I>,
}
