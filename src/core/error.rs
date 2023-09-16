use std::{
    error::Error as StdErr,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub enum Error<E>
where
    E: Debug + Display,
{
    RepositoryError(E),
    StoreError(E),
    ServiceError(E),
}

impl<E> Display for Error<E>
where
    E: Debug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepositoryError(e) => write!(f, "RepositoryError: {}", e),
            Self::StoreError(e) => write!(f, "StoreError: {}", e),
            Self::ServiceError(e) => write!(f, "ServiceError: {}", e),
        }
    }
}

impl<E> StdErr for Error<E> where E: Debug + Display {}
