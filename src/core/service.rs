use super::{repository::Repository, store::Store};
use crate::core::entities::{UploadedFile, UploadedFileCreate};
use std::error::Error;

pub struct Service<R, S>
where
    R: Repository,
    S: Store,
{
    repository: R,
    store: S,
}

impl<R, S, T> Service<R, S>
where
    R: Repository<Token = T>,
    S: Store<Token = T>,
{
    pub fn new(repository: R, store: S) -> Self {
        Self { repository, store }
    }

    pub async fn upload(&mut self, data: &[u8], filename: &str, uploader_id: R::ID) -> Result<R::ID, Box<dyn Error>> {
        let fetch_token = self.store.put(data).await?;
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
}
