use crate::core::entities::{UploadedFile, UploadedFileCreate, UploadedFileQuery};
use anyhow::Error;

pub struct Pagination {
    pub page: i64,
    pub size: i64,
}

pub trait Repository<ID> {
    async fn insert_uploaded_file(&self, file: UploadedFileCreate<ID>) -> Result<ID, Error>;
    async fn get_uploaded_file(&self, id: ID) -> Result<UploadedFile<ID>, Error>;
    async fn query_uploaded_files(&self, query: UploadedFileQuery<ID>, limit: Option<i64>, offset: Option<i64>) -> Result<(Vec<UploadedFile<ID>>, i64), Error>;
}
