#![feature(async_fn_in_trait)]

use core::{repository::Repository, store::Store};

use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    web::{get, post, scope},
    App, FromRequest,
};
use bytes::Bytes;
use futures::Stream;
use serde::Serialize;
use std::error::Error;

pub mod core;
pub mod handlers;
pub mod impls;

pub fn register_handlers<R, S, T, ID, TK, ST>(app: App<T>, base_path: &str) -> App<T>
where
    ID: FromRequest + Serialize + Clone + 'static,
    TK: 'static,
    R: Repository<Token = TK, ID = ID> + 'static,
    S: Store<Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin>, Token = TK> + 'static,
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    app.service(
        scope(base_path)
            .route("", post().to(handlers::upload::<ID, R, S, TK>))
            .route("/{id}", get().to(handlers::download::<ID, R, S, TK>)),
    )
}
