use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Error<E>(pub E)
where
    E: Debug + Display;

impl<E> Display for Error<E>
where
    E: Debug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "store error: {}", self.0)
    }
}

impl<E> std::error::Error for Error<E> where E: Debug + Display {}
