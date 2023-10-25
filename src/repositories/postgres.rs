use anyhow::Error;
use chrono::{DateTime, Utc};
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
    async fn get_uploaded_file(&self, id: &str) -> Result<UploadedFile, Error> {
        let (id, filename, mime_type, filepath, uploader_id, uploaded_at): (String, String, String, String, String, DateTime<Utc>) =
            query_as("SELECT * FROM uploaded_files WHERE id = $1").bind(id).fetch_one(&self.pool).await?;
        Ok(UploadedFile {
            id,
            filename,
            mime_type,
            filepath,
            uploader_id,
            uploaded_at,
        })
    }

    async fn insert_uploaded_file(&self, file: UploadedFileCreate) -> Result<String, Error> {
        let id = query_scalar("INSERT INTO uploaded_files (filename, mime_type, fetch_token, uploader_id) VALUES ($1, $2, $3, $4) RETURNING id")
            .bind(file.filename)
            .bind(file.mime_type)
            .bind(file.filepath)
            .bind(file.uploader_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(id)
    }
}
