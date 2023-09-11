use super::{repository::Repository, store::Store};
use crate::core::entities::{UploadedFile, UploadedFileCreate};
use actix_web::FromRequest;
use std::error::Error;
use std::pin::Pin;

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

    pub async fn upload(&mut self, stream: S::Stream, filename: &str, uploader_id: R::ID) -> Result<R::ID, Box<dyn Error>> {
        let fetch_token = self.store.put(stream).await?;
        self.repository
            .insert_uploaded_file(UploadedFileCreate {
                filename: filename.into(),
                mime_type: "".into(),
                fetch_token,
                uploader_id,
            })
            .await
    }

    pub async fn get_uploaded_file(&mut self, id: R::ID) -> Result<UploadedFile<R::ID, T>, Box<dyn Error>> {
        self.repository.get_uploaded_file(id).await
    }

    pub async fn download(&mut self, id: R::ID) -> Result<S::Stream, Box<dyn Error>> {
        let file = self.repository.get_uploaded_file(id).await?;
        self.store.get(&file.fetch_token).await
    }
}

impl<R, S> FromRequest for Service<R, S>
where
    R: Repository + FromRequest,
    R::Future: 'static,
    S: Store + FromRequest,
    S::Future: 'static,
{
    type Error = Box<dyn Error>;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let repository = R::from_request(req, payload);
        let store = S::from_request(req, payload);
        Box::pin(async move {
            Ok(Self {
                repository: repository.await.map_err(|e| e.into())?,
                store: store.await.map_err(|e| e.into())?,
            })
        })
    }
}
