#![feature(async_fn_in_trait)]

use core::{repository::Repository, service::Service, store::Store};

use actix_multipart::Field;
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    web::{post, Data},
    App, FromRequest,
};
use bytes::Bytes;
use futures::Stream;
use serde::Serialize;
use std::cell::RefCell;
use std::error::Error;

pub mod core;
pub mod handlers;
pub mod impls;

pub fn register_handlers<R, S, T, ID, TK, ST>(app: App<T>, base_path: &str, repository: R, store: S) -> App<T>
where
    ID: FromRequest + Serialize + Clone + 'static,
    TK: 'static,
    R: Repository<Token = TK, ID = ID> + 'static,
    S: Store<Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin>, Token = TK> + 'static,
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    let service = Service::new(repository, store);
    app.app_data(Data::new(RefCell::new(service))).route(base_path, post().to(handlers::upload::<ID, R, S, TK>))
}
