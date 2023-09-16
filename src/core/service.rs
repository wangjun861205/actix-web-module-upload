use super::{repository::Repository, store::Store};
use crate::core::entities::{UploadedFile, UploadedFileCreate};
use mime_guess::{self, mime};
use std::error::Error;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Service<R, S, ID, TK>
where
    R: Repository<ID, TK> + Clone,
    S: Store<TK> + Clone,
{
    repository: R,
    store: S,
    _phantom: PhantomData<(ID, TK)>,
}

impl<R, S, ID, TK> Service<R, S, ID, TK>
where
    R: Repository<ID, TK> + Clone,
    S: Store<TK> + Clone,
{
    pub fn new(repository: R, store: S) -> Self {
        Self {
            repository,
            store,
            _phantom: PhantomData,
        }
    }

    pub async fn upload(&self, stream: S::Stream, filename: &str, uploader_id: ID, token: TK, size_limit: Option<i64>) -> Result<ID, Box<dyn Error>> {
        let mime_type = match mime_guess::from_path(filename).first() {
            Some(mime_type) => mime_type,
            None => mime::APPLICATION_OCTET_STREAM,
        };
        let fetch_token = self.store.put(stream, token, size_limit).await?;
        self.repository
            .insert_uploaded_file(UploadedFileCreate {
                filename: filename.into(),
                mime_type: mime_type.to_string(),
                fetch_token,
                uploader_id,
            })
            .await
    }

    pub async fn get_uploaded_file(&self, id: ID) -> Result<UploadedFile<ID, TK>, Box<dyn Error>> {
        self.repository.get_uploaded_file(id).await
    }

    pub async fn download(&self, id: ID) -> Result<S::Stream, Box<dyn Error>> {
        let file = self.repository.get_uploaded_file(id).await?;
        self.store.get(&file.fetch_token).await
    }
}
