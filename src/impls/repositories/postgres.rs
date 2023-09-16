use chrono::{DateTime, Local};
use sqlx::postgres::PgPool;
use sqlx::{query_as, query_scalar, Postgres, QueryBuilder};
use std::error::Error as StdErr;

use crate::core::{
    entities::{UploadedFile, UploadedFileCreate, UploadedFileQuery},
    repository::Repository,
};

#[derive(Debug, Clone)]
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

type UploadedFileList<ID, TK> = Vec<UploadedFile<ID, TK>>;

impl<ID, TK> Repository<ID, TK> for PostgresRepository
where
    for<'i> ID: sqlx::Decode<'i, Postgres> + sqlx::Type<Postgres> + sqlx::Encode<'i, Postgres> + Send + Unpin + Clone,
    for<'t> TK: sqlx::Decode<'t, Postgres> + sqlx::Type<Postgres> + sqlx::Encode<'t, Postgres> + Send + Unpin,
{
    async fn get_uploaded_file(&self, id: ID) -> Result<UploadedFile<ID, TK>, Box<dyn StdErr>> {
        let (id, filename, mime_type, fetch_token, uploader_id, uploaded_at): (ID, String, String, TK, ID, DateTime<Local>) =
            query_as("SELECT * FROM uploaded_files WHERE id = $1").bind(id).fetch_one(&self.pool).await?;
        Ok(UploadedFile {
            id,
            filename,
            mime_type,
            fetch_token,
            uploader_id,
            uploaded_at,
        })
    }

    async fn insert_uploaded_file(&self, file: UploadedFileCreate<ID, TK>) -> Result<ID, Box<dyn StdErr>> {
        let id = query_scalar("INSERT INTO uploaded_files (filename, mime_type, fetch_token, uploader_id) VALUES ($1, $2, $3, $4) RETURNING id")
            .bind(file.filename)
            .bind(file.mime_type)
            .bind(file.fetch_token)
            .bind(file.uploader_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(id)
    }

    async fn query_uploaded_files(&self, query: UploadedFileQuery<ID>, limit: Option<i64>, offset: Option<i64>) -> Result<(Vec<UploadedFile<ID, TK>>, i64), Box<dyn StdErr>> {
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT * FROM uploaded_files 
        WHERE $1 IS NULL OR id = $1
        AND $2 IS NULL OR uploader_id = $2",
        );
        let count: i64 = q.build_query_scalar().bind(query.id_eq.clone()).bind(query.uploader_id_eq.clone()).fetch_one(&self.pool).await?;
        q.push(" LIMIT $3 OFFSET $4");
        let files: Result<UploadedFileList<ID, TK>, Box<dyn StdErr>> = query_as::<_, (ID, String, String, TK, ID, DateTime<Local>)>(&q.into_sql())
            .bind(query.id_eq)
            .bind(query.uploader_id_eq)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
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
