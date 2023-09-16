use crate::core::store::Store;
use crate::impls::stores::error::Error as StoreError;
use actix_web::{web::Data, FromRequest};
use bytes::Bytes;
use futures::{future::ready, Stream, StreamExt, TryStreamExt};
use std::error::Error;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::codec::{BytesCodec, FramedRead};
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

    async fn put(&mut self, stream: Self::Stream, size_limit: Option<i64>) -> Result<Self::Token, Box<dyn std::error::Error>> {
        let token = Uuid::new_v4().to_string();
        let mut file = File::create(format!("{}/{}", self.path, token)).await?;
        let mut curr_size = 0;
        let mut stream = stream.map(|bs| {
            if let Ok(bs) = &bs {
                curr_size += bs.len() as i64;
                if let Some(limit) = size_limit {
                    if curr_size > limit {
                        return Err(Box::new(StoreError("Size limit exceeded".to_owned())) as Box<dyn Error>);
                    }
                }
            }
            bs
        });
        while let Some(bs) = stream.try_next().await? {
            file.write_all(&bs).await?;
        }
        Ok(token)
    }

    async fn get(&mut self, token: &Self::Token) -> Result<Self::Stream, Box<dyn std::error::Error>> {
        let file = File::open(format!("{}/{}", self.path, token)).await?;
        let stream = FramedRead::new(file, BytesCodec::new()).map(|v| match v {
            Ok(b) => Ok(b.freeze()),
            Err(e) => Err(Box::new(e) as Box<dyn Error>),
        });
        Ok(Box::new(stream))
    }
}

use crate::impls::stores::common::StorePath;

impl FromRequest for LocalFSStore {
    type Error = Box<dyn Error>;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(path) = req.app_data::<Data<StorePath>>() {
            return futures::future::ready(Ok(Self::new(path.0.to_string())));
        }
        futures::future::ready(Err(Box::new(StoreError("Store path not found"))))
    }
}
