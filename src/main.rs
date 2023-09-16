#![feature(async_fn_in_trait)]

use core::{repository::Repository, store::Store};

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
    let pool = PgPool::connect(&dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(StorePath("/tmp/uploaded_files".to_owned())))
            .route("/", post().to(handlers::upload::<i32, PostgresRepository<PgPool>, LocalFSStore, String>))
            .route("/{id}", get().to(handlers::download::<i32, PostgresRepository<PgPool>, LocalFSStore, String>))
    })
    .bind(dotenv::var("ADDRESS").expect("ADDRESS must be set"))?
    .run()
    .await
}
