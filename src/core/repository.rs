use crate::core::entities::{UploadedFile, UploadedFileCreate};
use anyhow::Error;

pub trait Repository {
    async fn insert_uploaded_file(&self, file: UploadedFileCreate) -> Result<String, Error>;
    async fn get_uploaded_file(&self, id: &str) -> Result<Option<UploadedFile>, Error>;
}
