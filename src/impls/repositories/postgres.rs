use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use chrono::{DateTime, Local};
use sqlx::postgres::PgExecutor;
use sqlx::{query_as, query_scalar, Postgres, QueryBuilder};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use crate::core::{
    entities::{UploadedFile, UploadedFileCreate, UploadedFileQuery},
    repository::Repository,
};

pub struct PostgresRepository<E>
where
    for<'e> &'e E: PgExecutor<'e>,
{
    executor: Data<E>,
}

impl<E> PostgresRepository<E>
where
    for<'e> &'e E: PgExecutor<'e>,
{
    pub fn new(executor: Data<E>) -> Self {
        Self { executor }
    }
}

type UploadedFileList = Vec<UploadedFile<i32, String>>;

impl<E> Repository for PostgresRepository<E>
where
    for<'e> &'e E: PgExecutor<'e>,
    E: 'static,
{
    type ID = i32;
    type Token = String;

    async fn get_uploaded_file(&mut self, id: Self::ID) -> Result<UploadedFile<Self::ID, Self::Token>, Box<dyn Error>> {
        let (id, filename, mime_type, fetch_token, uploader_id, uploaded_at): (i32, String, String, String, i32, DateTime<Local>) =
            query_as("SELECT * FROM uploaded_files WHERE id = $1").bind(id).fetch_one(self.executor.as_ref()).await?;
        Ok(UploadedFile {
            id,
            filename,
            mime_type,
            fetch_token,
            uploader_id,
            uploaded_at,
        })
    }

    async fn insert_uploaded_file(&mut self, file: UploadedFileCreate<Self::ID, Self::Token>) -> Result<Self::ID, Box<dyn Error>> {
        let id = query_scalar("INSERT INTO uploaded_files (filename, mime_type, fetch_token, uploader_id) VALUES ($1, $2, $3, $4) RETURNING id")
            .bind(file.filename)
            .bind(file.mime_type)
            .bind(file.fetch_token)
            .bind(file.uploader_id)
            .fetch_one(self.executor.as_ref())
            .await?;
        Ok(id)
    }

    async fn query_uploaded_files(&mut self, query: UploadedFileQuery<Self::ID>, limit: Option<i64>, offset: Option<i64>) -> Result<(Vec<UploadedFile<Self::ID, Self::Token>>, i64), Box<dyn Error>> {
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT * FROM uploaded_files 
        WHERE $1 IS NULL OR id = $1
        AND $2 IS NULL OR uploader_id = $2",
        );
        let count: i64 = q.build_query_scalar().bind(query.id_eq).bind(query.uploader_id_eq).fetch_one(self.executor.as_ref()).await?;
        q.push(" LIMIT $3 OFFSET $4");
        let files: Result<UploadedFileList, Box<dyn Error>> = query_as::<_, (i32, String, String, String, i32, DateTime<Local>)>(&q.into_sql())
            .bind(query.id_eq)
            .bind(query.uploader_id_eq)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.executor.as_ref())
            .await?
            .into_iter()
            .map(|(id, filename, mime_type, fetch_token, uploader_id, uploaded_at)| {
                Ok(UploadedFile {
                    id,
                    filename,
                    mime_type,
                    fetch_token,
                    uploader_id,
                    uploaded_at,
                })
            })
            .collect();
        Ok((files?, count))
    }
}

impl<E> FromRequest for PostgresRepository<E>
where
    for<'e> &'e E: PgExecutor<'e>,
    E: 'static,
{
    type Error = Box<dyn Error>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = Data::<E>::from_request(req, payload);
        Box::pin(async move {
            let executor = fut.await?;
            Ok(Self::new(executor))
        })
    }
}
