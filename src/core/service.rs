use super::{repository::Repository, store::Store};
use crate::core::entities::{UploadedFile, UploadedFileCreate};
use anyhow::Error;
use mime_guess::{self, mime};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Service<R, S, ID>
where
    R: Repository<ID> + Clone,
    S: Store + Clone,
{
    repository: R,
    store: S,
    _phantom: PhantomData<ID>,
}

impl<R, S, ID> Service<R, S, ID>
where
    R: Repository<ID> + Clone,
    S: Store + Clone,
    ID: PartialEq,
{
    pub fn new(repository: R, store: S) -> Self {
        Self {
            repository,
            store,
            _phantom: PhantomData,
        }
    }

    pub async fn upload(&self, stream: S::Stream, filename: &str, uploader_id: ID, size_limit: Option<i64>) -> Result<ID, Error> {
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
                uploader_id,
            })
            .await
    }

    pub async fn get_uploaded_file(&self, id: ID) -> Result<UploadedFile<ID>, Error> {
        self.repository.get_uploaded_file(id).await
    }

    pub async fn download(&self, id: ID) -> Result<S::Stream, Error> {
        let file = self.repository.get_uploaded_file(id).await?;
        self.store.get(&file.filepath).await
    }

    pub async fn is_owner(&self, id: ID, uploader_id: ID) -> Result<bool, Error> {
        let file = self.repository.get_uploaded_file(id).await?;
        Ok(file.uploader_id == uploader_id)
    }
}
