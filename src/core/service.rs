use super::{repository::Repository, store::Store};
use crate::core::entities::{UploadedFile, UploadedFileCreate};
use anyhow::Error;
use bytes::Bytes;
use chrono::Utc;
use futures::Stream;
use mime_guess::{self, mime};
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct Service<R, S>
where
    R: Repository + Clone,
    S: Store + Clone,
{
    repository: R,
    store: S,
}

impl<R, S> Service<R, S>
where
    R: Repository + Clone,
    S: Store + Clone,
{
    pub fn new(repository: R, store: S) -> Self {
        Self { repository, store }
    }

    pub async fn upload(&self, stream: impl Stream<Item = Result<Bytes, Error>>, filename: &str, uploader_id: &str, size_limit: Option<i64>) -> Result<String, Error> {
        let mime_type = match mime_guess::from_path(filename).first() {
            Some(mime_type) => mime_type,
            None => mime::APPLICATION_OCTET_STREAM,
        };
        let filepath = self.store.put(stream, size_limit).await?;
        self.repository
            .insert_uploaded_file(UploadedFileCreate {
                filename: filename.into(),
                mime_type: mime_type.to_string(),
                filepath,
                uploader_id: uploader_id.into(),
                uploaded_at: Utc::now(),
            })
            .await
    }

    pub async fn get_uploaded_file(&self, id: &str) -> Result<UploadedFile, Error> {
        self.repository.get_uploaded_file(id).await
    }

    pub async fn download(&self, id: &str) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>>, Error> {
        let file = self.repository.get_uploaded_file(id).await?;
        self.store.get(&file.filepath).await
    }

    pub async fn is_owner(&self, id: &str, uploader_id: &str) -> Result<bool, Error> {
        let file = self.repository.get_uploaded_file(id).await?;
        Ok(file.uploader_id == uploader_id)
    }
}
