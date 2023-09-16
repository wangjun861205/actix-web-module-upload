use crate::core::{repository::Repository, service::Service, store::Store};
use actix_multipart::Multipart;
use actix_web::{
    http::StatusCode,
    web::{Json, Path},
    HttpRequest, HttpResponse,
};
use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
use serde::Serialize;
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

pub async fn upload<I, R, S, TK>(req: HttpRequest, mut service: Service<R, S>, mut payload: Multipart) -> Result<Json<UploadResponse<I>>, Box<dyn Error>>
where
    R: Repository<Token = TK, ID = I>,
    S: Store<Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin>, Token = TK>,
    I: Serialize + Clone + TryFromStr,
{
    if let Some(uid) = req.headers().get("X-User-ID") {
        if let Ok(uid) = uid.to_str() {
            let uid: I = I::try_from_str(uid)?;
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

pub async fn download<I, R, S, TK>(mut service: Service<R, S>, id: Path<(I,)>) -> Result<HttpResponse, Box<dyn Error>>
where
    R: Repository<Token = TK, ID = I>,
    S: Store<Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin>, Token = TK>,
    I: Serialize + Clone,
{
    let info = service.get_uploaded_file(id.clone().0).await?;
    Ok(HttpResponse::build(StatusCode::OK).content_type(info.mime_type).streaming(service.download(id.clone().0).await?))
}
