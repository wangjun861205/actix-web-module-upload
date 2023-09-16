use crate::core::entities::{UploadedFile, UploadedFileCreate, UploadedFileQuery};
use std::error::Error;

pub struct Pagination {
    pub page: i64,
    pub size: i64,
}

pub trait Repository<ID, TK> {
    async fn insert_uploaded_file(&self, file: UploadedFileCreate<ID, TK>) -> Result<ID, Box<dyn Error>>;
    async fn get_uploaded_file(&self, id: ID) -> Result<UploadedFile<ID, TK>, Box<dyn Error>>;
    async fn query_uploaded_files(&self, query: UploadedFileQuery<ID>, limit: Option<i64>, offset: Option<i64>) -> Result<(Vec<UploadedFile<ID, TK>>, i64), Box<dyn Error>>;
}
