#![feature(async_fn_in_trait)]

use core::{repository::Repository, service::Service, store::Store};

use crate::handlers::TryFromStr;
use actix_web::{
    middleware::Logger,
    web::{get, post, Data},
    App, HttpServer,
};
use bytes::Bytes;
use from_env::{FromEnv, FromEnvDerive};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

pub mod core;
pub mod handlers;
pub mod impls;

impl TryFromStr for i32 {
    fn try_from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        s.parse().map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

impl TryFromStr for String {
    fn try_from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.to_owned())
    }
}

#[derive(FromEnvDerive)]
struct Config {
    log_level: String,
    address: String,
}

pub async fn start<RP, ST, ID, TK>(service: Service<RP, ST, ID, TK>) -> std::io::Result<()>
where
    RP: Repository<ID, TK> + Clone + Send + Unpin,
    ST: Store<TK, Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin + 'static>> + Clone + Send + Unpin,
    for<'de> ID: Serialize + Deserialize<'de> + TryFromStr + Clone + Send + Unpin,
    TK: Serialize + TryFromStr + Display + Clone + Send + Unpin,
{
    dotenv::dotenv().ok();
    let config = Config::from_env();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(config.log_level));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%t %a %r %{X-User-ID}i %s %T"))
            .app_data(Data::new(service.clone()))
            .route("/", post().to(handlers::upload::<RP, ST, ID, TK>))
            .route("/{id}", get().to(handlers::download::<RP, ST, ID, TK>))
    })
    .bind(config.address)?
    .run()
    .await
}
