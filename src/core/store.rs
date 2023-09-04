use std::error::Error;

pub trait Store {
    type Token;

    async fn put(&mut self, data: &[u8]) -> Result<Self::Token, Box<dyn Error>>;
    async fn get(&mut self, token: &Self::Token) -> Result<Vec<u8>, Box<dyn Error>>;
}
