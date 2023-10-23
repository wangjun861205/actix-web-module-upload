use crate::core::store::Store;
use anyhow::Error;
use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::codec::{BytesCodec, FramedRead};
use uuid::Uuid;

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

impl Store for LocalFSStore {
    type Stream = Box<dyn Stream<Item = Result<Bytes, Error>> + Unpin>;

    async fn put(&self, stream: Self::Stream, size_limit: Option<i64>) -> Result<String, Error> {
        let token = Uuid::new_v4().to_string();
        let filepath = format!("{}/{}", self.path, token);
        let mut file = File::create(&filepath).await?;
        let mut curr_size = 0;
        let mut stream = stream.map(|bs| {
            if let Ok(bs) = &bs {
                curr_size += bs.len() as i64;
                if let Some(limit) = size_limit {
                    if curr_size > limit {
                        return Err(Error::msg("Size limit exceeded"));
                    }
                }
            }
            bs
        });
        while let Some(bs) = stream.try_next().await? {
            file.write_all(&bs).await?;
        }
        Ok(filepath)
    }

    async fn get(&self, filepath: &str) -> Result<Self::Stream, Error> {
        let file = File::open(filepath).await?;
        let stream = FramedRead::new(file, BytesCodec::new()).map(|v| match v {
            Ok(b) => Ok(b.freeze()),
            Err(e) => Err(Error::new(e)),
        });
        Ok(Box::new(stream))
    }
}
