use anyhow::Error;
use bytes::Bytes;
use futures::Stream;

pub trait Store {
    async fn put(&self, stream: impl Stream<Item = Result<Bytes, Error>>, size_limit: Option<i64>) -> Result<String, Error>;
    async fn get(&self, filepath: &str) -> Result<Box<dyn Stream<Item = Result<Bytes, Error>>>, Error>;
}
