use actix_web::FromRequest;

use crate::core::entities::{UploadedFile, UploadedFileCreate, UploadedFileQuery};
use std::error::Error;

pub struct Pagination {
    pub page: i64,
    pub size: i64,
}

pub trait Repository: FromRequest {
    type ID;
    type Token;

    async fn insert_uploaded_file(&mut self, file: UploadedFileCreate<Self::ID, Self::Token>) -> Result<Self::ID, Box<dyn Error>>;
    async fn get_uploaded_file(&mut self, id: Self::ID) -> Result<UploadedFile<Self::ID, Self::Token>, Box<dyn Error>>;
    async fn query_uploaded_files(&mut self, query: UploadedFileQuery<Self::ID>, limit: Option<i64>, offset: Option<i64>) -> Result<(Vec<UploadedFile<Self::ID, Self::Token>>, i64), Box<dyn Error>>;
}
