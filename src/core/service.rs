use super::{repository::Repository, store::Store};
use crate::core::entities::{UploadedFile, UploadedFileCreate};
use anyhow::Error;
use bytes::Bytes;
use chrono::Utc;
use futures::Stream;
use mime_guess::{self, mime};
use serde::Serialize;
use std::pin::Pin;

#[derive(Debug)]
pub struct Service<R, S>
where
    R: Repository,
    S: Store,
{
    repository: R,
    store: S,
}

#[derive(Debug, Clone, Serialize)]
pub struct UploadResp {
    id: String,
    mime_type: String,
}

impl<R, S> Service<R, S>
where
    R: Repository,
    S: Store,
{
    pub fn new(repository: R, store: S) -> Self {
        Self { repository, store }
    }

    pub async fn upload(
        &self,
        stream: impl Stream<Item = Result<Bytes, Error>>,
        filename: &str,
        uploader_id: &str,
        size_limit: Option<i64>,
    ) -> Result<UploadResp, Error> {
        let mime_type = match mime_guess::from_path(filename).first() {
            Some(mime_type) => mime_type,
            None => mime::APPLICATION_OCTET_STREAM,
        };
        let stored_name = self.store.put(stream, size_limit).await?;
        let id = self
            .repository
            .insert_uploaded_file(UploadedFileCreate {
                origin_name: filename.into(),
                mime_type: mime_type.to_string(),
                stored_name,
                uploader_id: uploader_id.into(),
                uploaded_at: Utc::now(),
            })
            .await?;
        Ok(UploadResp {
            id,
            mime_type: mime_type.to_string(),
        })
    }

    pub async fn get_uploaded_file(&self, id: &str) -> Result<Option<UploadedFile>, Error> {
        self.repository.get_uploaded_file(id).await
    }

    pub async fn download(&self, id: &str) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>>, Error> {
        if let Some(file) = self.repository.get_uploaded_file(id).await? {
            return self.store.get(&file.stored_name).await;
        }
        Err(Error::msg("File not found"))
    }

    pub async fn is_owner(&self, id: &str, uploader_id: &str) -> Result<bool, Error> {
        if let Some(file) = self.repository.get_uploaded_file(id).await? {
            return Ok(file.uploader_id == uploader_id);
        }
        Err(Error::msg("File not found"))
    }
}
