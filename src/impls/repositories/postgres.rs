use actix_web::FromRequest;
use sqlx::postgres::PgExecutor;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use crate::core::{
    entities::UploadedFileQuery,
    repository::{Pagination, Repository},
};

pub struct PostgresRepository<E>
where
    E: for<'e> PgExecutor<'e>,
{
    executor: E,
}

impl<E> PostgresRepository<E>
where
    E: for<'e> PgExecutor<'e>,
{
    pub fn new(executor: E) -> Self {
        Self { executor }
    }
}

impl<E> Repository for PostgresRepository<E>
where
    E: for<'e> PgExecutor<'e>,
{
    type ID = i32;
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

impl<E> FromRequest for PostgresRepository<E>
where
    E: for<'e> PgExecutor<'e>,
{
    type Error = Box<dyn Error>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        unimplemented!()
    }
}
