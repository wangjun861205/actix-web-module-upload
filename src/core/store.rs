use bytes::Bytes;
use futures::Stream;
use std::error::Error;

pub trait Store<TK> {
    type Stream: Stream<Item = Result<Bytes, Box<dyn Error>>>;

    async fn put(&mut self, stream: Self::Stream, token: TK, size_limit: Option<i64>) -> Result<TK, Box<dyn Error>>;
    async fn get(&mut self, token: &TK) -> Result<Self::Stream, Box<dyn Error>>;
}
