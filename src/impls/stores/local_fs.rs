use crate::core::store::Store;
use actix_multipart::Field;
use bytes::Bytes;
use futures::{Sink, SinkExt, Stream, StreamExt, TryStreamExt};
use std::error::Error;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

pub struct LocalFSStore {
    path: String,
}

impl LocalFSStore {
    pub fn new<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        Self { path: path.into() }
    }
}

impl Store for LocalFSStore {
    type Token = String;
    type Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin>;

    async fn put(&mut self, mut stream: Self::Stream) -> Result<Self::Token, Box<dyn std::error::Error>> {
        let token = Uuid::new_v4().to_string();
        let mut file = File::create(format!("{}/{}", self.path, token)).await?;
        while let Some(bs) = stream.try_next().await? {
            file.write_all(&bs).await?;
        }
        Ok(token)
    }

    async fn get(&mut self, token: &Self::Token) -> Result<Self::Stream, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
