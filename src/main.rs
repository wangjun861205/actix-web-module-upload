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
use impls::stores::common::StorePath;
use sqlx::PgPool;
use std::error::Error;

pub mod core;
pub mod handlers;
pub mod impls;

impl TryFromStr for i32 {
    fn try_from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        s.parse().map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to database");
    let store_path = dotenv::var("STORE_PATH").expect("STORE_PATH must be set");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(StorePath(store_path.clone())))
            .route("/", post().to(handlers::upload::<PostgresRepository<&PgPool>, LocalFSStore, i32, String>))
            .route("/{id}", get().to(handlers::download::<PostgresRepository<&PgPool>, LocalFSStore, i32, String>))
    })
    .bind(dotenv::var("ADDRESS").expect("ADDRESS must be set"))?
    .run()
    .await
}
