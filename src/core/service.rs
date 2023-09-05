use super::{repository::Repository, store::Store};
use crate::core::entities::{UploadedFile, UploadedFileCreate};
use bytes::Bytes;
use futures::Stream;
use std::error::Error;
use std::marker::PhantomData;

pub struct Service<R, S, ST>
where
    R: Repository,
    S: Store,
    ST: Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin,
{
    repository: R,
    store: S,
    _phantom: PhantomData<ST>,
}

impl<R, S, T, ST> Service<R, S, ST>
where
    R: Repository<Token = T>,
    S: Store<Token = T, Stream = ST>,
    ST: Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin,
{
    pub fn new(repository: R, store: S) -> Self {
        Self {
            repository,
            store,
            _phantom: PhantomData,
        }
    }

    pub async fn upload(&mut self, stream: ST, filename: &str, uploader_id: R::ID) -> Result<R::ID, Box<dyn Error>> {
        let fetch_token = self.store.put(stream).await?;
        Ok(self
            .repository
            .insert_uploaded_file(UploadedFileCreate {
                filename: filename.into(),
                mime_type: "".into(),
                fetch_token,
                uploader_id,
            })
            .await?)
    }

    pub async fn get_uploaded_file(&mut self, id: R::ID) -> Result<UploadedFile<R::ID, T>, Box<dyn Error>> {
        self.repository.get_uploaded_file(id).await
    }

    pub async fn download(&mut self, id: R::ID) -> Result<ST, Box<dyn Error>> {
        let file = self.repository.get_uploaded_file(id).await?;
        Ok(self.store.get(&file.fetch_token).await?)
    }
}
