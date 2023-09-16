use crate::core::{repository::Repository, service::Service, store::Store};
use crate::impls::repositories::postgres::PostgresRepository;
use crate::impls::stores::common::StorePath;
use crate::impls::stores::local_fs::LocalFSStore;
use actix_multipart::Multipart;
use actix_web::{
    http::StatusCode,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
use serde::Serialize;
use sqlx::{pool::PoolConnection, PgPool};
use sqlx::{Executor, PgConnection, Postgres};
use std::error::Error;

pub trait TryFromStr
where
    Self: Sized,
{
    fn try_from_str(s: &str) -> Result<Self, Box<dyn Error>>;
}

#[derive(Debug, Serialize)]
pub struct UploadResponse<I>
where
    I: Serialize,
{
    ids: Vec<I>,
}

pub async fn upload<R, S, ID, TK>(req: HttpRequest, mut payload: Multipart, pool: Data<PgPool>, store_path: Data<StorePath>) -> Result<Json<UploadResponse<ID>>, Box<dyn Error>>
where
    R: Repository<ID, TK>,
    S: Store<Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin>, Token = TK>,
    for<'i> ID: sqlx::Decode<'i, Postgres> + sqlx::Encode<'i, Postgres> + sqlx::Type<Postgres> + Serialize + Clone + TryFromStr + Send + Unpin + 'static,
{
    let repository = PostgresRepository::new(pool.as_ref());
    let store = LocalFSStore::new(store_path.0.clone());
    let mut service = Service::new(repository, store);
    if let Some(uid) = req.headers().get("X-User-ID") {
        if let Ok(uid) = uid.to_str() {
            let uid: ID = ID::try_from_str(uid)?;
            let mut ids = vec![];
            while let Ok(Some(field)) = payload.try_next().await {
                if field.content_disposition().get_filename().is_none() {
                    continue;
                }
                let filename = field.content_disposition().get_filename().unwrap().to_owned();
                let trans = Box::new(field.map(|res| res.map_err(|e| e.into())));
                let size_limit = req.headers().get("X-Size-Limit").map(|s| s.to_str().unwrap_or("-1").parse().unwrap_or(-1));
                ids.push(service.upload(trans, &filename, uid.clone(), size_limit).await?);
            }
            return Ok(Json(UploadResponse { ids }));
        }
    }
    Err("Invalid user".into())
}

pub async fn download<R, S, ID, TK>(pool: Data<PgPool>, store_path: Data<StorePath>, id: Path<(ID,)>) -> Result<HttpResponse, Box<dyn Error>>
where
    R: Repository<ID, TK>,
    S: Store<Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin>, Token = TK>,
    for<'i> ID: sqlx::Decode<'i, Postgres> + sqlx::Encode<'i, Postgres> + sqlx::Type<Postgres> + Serialize + Clone + TryFromStr + Send + Unpin + 'static,
{
    let repository = PostgresRepository::new(pool.as_ref());
    let store = LocalFSStore::new(store_path.0.clone());
    let mut service = Service::new(repository, store);
    let info = service.get_uploaded_file(id.clone().0).await?;
    Ok(HttpResponse::build(StatusCode::OK).content_type(info.mime_type).streaming(service.download(id.clone().0).await?))
}
