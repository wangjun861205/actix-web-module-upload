use crate::core::{repository::Repository, service::Service, store::Store};
use actix_multipart::Multipart;
use actix_web::web::Json;
use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct UploadResponse<I>
where
    I: Serialize,
{
    ids: Vec<I>,
}

pub async fn upload<I, R, S, TK>(mut service: Service<R, S>, mut payload: Multipart, uploader_id: I) -> Result<Json<UploadResponse<I>>, Box<dyn Error>>
where
    R: Repository<Token = TK, ID = I>,
    S: Store<Stream = Box<dyn Stream<Item = Result<Bytes, Box<dyn Error>>> + Unpin>, Token = TK>,
    I: Serialize + Clone,
{
    let mut ids = vec![];
    while let Ok(Some(field)) = payload.try_next().await {
        if !field.content_disposition().is_attachment() {
            continue;
        }
        if field.content_disposition().get_filename().is_none() {
            continue;
        }
        let filename = field.content_disposition().get_filename().unwrap().to_owned();
        let trans = Box::new(field.map(|res| res.map_err(|e| e.into())));
        ids.push(service.upload(trans, &filename, uploader_id.clone()).await?);
    }
    Ok(Json(UploadResponse { ids }))
}
