use bytes::Bytes;
use futures::Stream;
use std::error::Error;

pub trait Store<S>
where
    S: Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin,
{
    type Token;

    async fn put(&mut self, stream: S) -> Result<Self::Token, Box<dyn Error>>;
    async fn get(&mut self, token: &Self::Token) -> Result<S, Box<dyn Error>>;
}
