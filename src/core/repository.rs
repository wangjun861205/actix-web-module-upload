use crate::core::entities::{UploadedFile, UploadedFileCreate, UploadedFileQuery};
use std::error::Error;

pub struct Pagination {
    pub page: i64,
    pub size: i64,
}

pub trait Repository {
    type ID;
    type Token;

    async fn insert_uploaded_file(&mut self, file: UploadedFileCreate<Self::ID, Self::Token>) -> Result<Self::ID, Box<dyn Error>>;
    async fn get_uploaded_file(&mut self, id: Self::ID) -> Result<UploadedFile<Self::ID, Self::Token>, Box<dyn Error>>;
    async fn query_uploaded_files(&mut self, query: UploadedFileQuery<Self::ID>, pagination: Option<Pagination>) -> Result<Vec<UploadedFile<Self::ID, Self::Token>>, i64>;
}
