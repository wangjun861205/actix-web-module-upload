use actix_web::{dev::Payload, FromRequest, HttpRequest};
use chrono::{DateTime, Local};
use sqlx::postgres::PgExecutor;
use sqlx::{query_as, query_scalar, Executor, PgPool, Postgres, QueryBuilder, Transaction};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;

use crate::core::{
    entities::{UploadedFile, UploadedFileCreate, UploadedFileQuery},
    repository::Repository,
};

pub struct PostgresRepository<E>
where
    for<'e> &'e E: PgExecutor<'e>,
{
    executor: E,
}

impl<E> PostgresRepository<E>
where
    for<'e> &'e E: PgExecutor<'e>,
{
    pub fn new(executor: E) -> Self {
        Self { executor }
    }
}

type UploadedFileList = Vec<UploadedFile<i32, String>>;

impl<E> Repository for PostgresRepository<E>
where
    for<'e> &'e E: PgExecutor<'e>,
    E: FromRequest,
    E::Error: Error + 'static,
    E::Future: 'static,
{
    type ID = i32;
    type Token = String;

    async fn get_uploaded_file(&mut self, id: Self::ID) -> Result<UploadedFile<Self::ID, Self::Token>, Box<dyn Error>> {
        let (id, filename, mime_type, fetch_token, uploader_id, uploaded_at): (i32, String, String, String, i32, String) =
            query_as("SELECT * FROM uploaded_files WHERE id = $1").bind(id).fetch_one(&self.executor).await?;
        let mut uploaded_at = DateTime::from_str(&uploaded_at)?;
        uploaded_at = uploaded_at.with_timezone(&Local);
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
        let id = query_scalar("INSERT INTO uploaded_files (filename, mime_type, fetch_token, uploader_id, uploaded_at) VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(file.filename)
            .bind(file.mime_type)
            .bind(file.fetch_token)
            .bind(file.uploader_id)
            .bind(chrono::Local::now().to_rfc3339())
            .fetch_one(&self.executor)
            .await?;
        Ok(id)
    }

    async fn query_uploaded_files(&mut self, query: UploadedFileQuery<Self::ID>, limit: Option<i64>, offset: Option<i64>) -> Result<(Vec<UploadedFile<Self::ID, Self::Token>>, i64), Box<dyn Error>> {
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT * FROM uploaded_files 
        WHERE $1 IS NULL OR id = $1
        AND $2 IS NULL OR uploader_id = $2",
        );
        let count: i64 = q.build_query_scalar().bind(query.id_eq).bind(query.uploader_id_eq).fetch_one(&self.executor).await?;
        q.push(" LIMIT $3 OFFSET $4");
        let files: Result<UploadedFileList, Box<dyn Error>> = query_as::<_, (i32, String, String, String, i32, String)>(&q.into_sql())
            .bind(query.id_eq)
            .bind(query.uploader_id_eq)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.executor)
            .await?
            .into_iter()
            .map(|(id, filename, mime_type, fetch_token, uploader_id, uploaded_at)| {
                let uploaded_at: DateTime<Local> = DateTime::from_str(&uploaded_at)?;
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
    E: FromRequest,
    E::Error: Error + 'static,
    E::Future: 'static,
{
    type Error = Box<dyn Error>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = E::from_request(req, payload);
        Box::pin(async move {
            let executor = fut.await?;
            Ok(Self::new(executor))
        })
    }
}
