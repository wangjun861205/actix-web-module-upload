use sqlx::postgres::PgPool;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::{
    entities::{UploadedFile, UploadedFileCreate, UploadedFileQuery},
    repository::{Pagination, Repository},
};

pub struct PostgresRepository {
    pool: Rc<RefCell<PgPool>>,
}

impl PostgresRepository {
    pub fn new(pool: Rc<RefCell<PgPool>>) -> Self {
        Self { pool }
    }
}

impl Repository for PostgresRepository {
    type ID = String;
    type Token = String;

    async fn get_uploaded_file(&mut self, id: Self::ID) -> Result<crate::core::entities::UploadedFile<Self::ID, Self::Token>, Box<dyn std::error::Error>> {
        unimplemented!()
    }

    async fn insert_uploaded_file(&mut self, file: crate::core::entities::UploadedFileCreate<Self::ID, Self::Token>) -> Result<Self::ID, Box<dyn std::error::Error>> {
        unimplemented!()
    }

    async fn query_uploaded_files(&mut self, query: UploadedFileQuery<Self::ID>, pagination: Option<Pagination>) -> Result<Vec<crate::core::entities::UploadedFile<Self::ID, Self::Token>>, i64> {
        unimplemented!()
    }
}
