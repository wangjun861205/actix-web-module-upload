use anyhow::Error;
use sqlx::postgres::PgPool;
use sqlx::{query_as, query_scalar};

use crate::core::{
    entities::{UploadedFile, UploadedFileCreate},
    repository::Repository,
};

#[derive(Debug, Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Repository for Postgres {
    async fn get_uploaded_file(&self, id: &str) -> Result<Option<UploadedFile>, Error> {
        if let Some((id, origin_name, mime_type, stored_name, uploader_id, uploaded_at)) =
            query_as("SELECT * FROM uploaded_files WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?
        {
            return Ok(Some(UploadedFile {
                id,
                origin_name,
                mime_type,
                stored_name,
                uploader_id,
                uploaded_at,
            }));
        }
        Ok(None)
    }

    async fn insert_uploaded_file(&self, file: UploadedFileCreate) -> Result<String, Error> {
        let id = query_scalar("INSERT INTO uploaded_files (origin_name, mime_type, stored_name, uploader_id) VALUES ($1, $2, $3, $4) RETURNING id")
            .bind(file.origin_name)
            .bind(file.mime_type)
            .bind(file.stored_name)
            .bind(file.uploader_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(id)
    }
}
