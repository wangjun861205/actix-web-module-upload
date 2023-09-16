use crate::core::store::Store;
use crate::impls::stores::error::Error as StoreError;
use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
use std::{error::Error, fmt::Display};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Debug, Clone)]
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

impl<TK> Store<TK> for LocalFSStore
where
    TK: Display,
{
    type Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin>;

    async fn put(&self, stream: Self::Stream, token: TK, size_limit: Option<i64>) -> Result<TK, Box<dyn std::error::Error>> {
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

    async fn get(&self, token: &TK) -> Result<Self::Stream, Box<dyn std::error::Error>> {
        let file = File::open(format!("{}/{}", self.path, token)).await?;
        let stream = FramedRead::new(file, BytesCodec::new()).map(|v| match v {
            Ok(b) => Ok(b.freeze()),
            Err(e) => Err(Box::new(e) as Box<dyn Error>),
        });
        Ok(Box::new(stream))
    }
}
