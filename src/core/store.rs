use bytes::Bytes;
use futures::Stream;
use std::error::Error;

pub trait Store {
    type Token;
    type Stream: Stream<Item = Result<Bytes, Box<dyn Error>>>;

    async fn put(&mut self, stream: Self::Stream) -> Result<Self::Token, Box<dyn Error>>;
    async fn get(&mut self, token: &Self::Token) -> Result<Self::Stream, Box<dyn Error>>;
}
