use anyhow::Error;
use bytes::Bytes;
use futures::Stream;

pub trait Store {
    type Stream: Stream<Item = Result<Bytes, Error>>;

    async fn put(&self, stream: Self::Stream, size_limit: Option<i64>) -> Result<String, Error>;
    async fn get(&self, filepath: &str) -> Result<Self::Stream, Error>;
}
