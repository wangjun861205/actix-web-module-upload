#![feature(async_fn_in_trait)]

use core::{repository::Repository, service::Service, store::Store};

use crate::{
    handlers::TryFromStr,
    impls::{repositories::postgres::PostgresRepository, stores::local_fs::LocalFSStore},
};
use actix_web::{
    web::{get, post, Data},
    App, HttpServer,
};
use bytes::Bytes;
use futures::Stream;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
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

pub async fn start<RP, ST, ID, TK>(service: Service<RP, ST, ID, TK>) -> std::io::Result<()>
where
    RP: Repository<ID, TK> + Clone + Send + Unpin,
    ST: Store<TK, Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin + 'static>> + Clone + Send + Unpin,
    for<'de> ID: Serialize + Deserialize<'de> + TryFromStr + Clone + Send + Unpin,
    TK: Serialize + TryFromStr + Display + Clone + Send + Unpin,
{
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(service.clone()))
            .route("/", post().to(handlers::upload::<RP, ST, ID, TK>))
            .route("/{id}", get().to(handlers::download::<RP, ST, ID, TK>))
    })
    .bind(dotenv::var("ADDRESS").expect("ADDRESS must be set"))?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to database");
    let store_path = dotenv::var("STORE_PATH").expect("STORE_PATH must be set");
    let service: Service<_, _, i32, String> = Service::new(PostgresRepository::new(pool.clone()), LocalFSStore::new(store_path));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(service.clone()))
            .route("/", post().to(handlers::upload::<PostgresRepository, LocalFSStore, i32, String>))
            .route("/{id}", get().to(handlers::download::<PostgresRepository, LocalFSStore, i32, String>))
    })
    .bind(dotenv::var("ADDRESS").expect("ADDRESS must be set"))?
    .run()
    .await
}
