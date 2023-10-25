use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOneOptions,
    Database,
};

use crate::core::{
    entities::{UploadedFile, UploadedFileCreate},
    repository::Repository,
};
use anyhow::Error;

#[derive(Debug, Clone)]
pub struct Mongo {
    db: Database,
}

impl Mongo {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

impl Repository for Mongo {
    async fn get_uploaded_file(&self, id: &str) -> Result<UploadedFile, Error> {
        self.db
            .collection("uploaded_files")
            .find_one(
                doc! { "_id": ObjectId::parse_str(id)? },
                FindOneOptions::builder()
                    .projection(doc! {
                        "id": {"$toString": "$_id"},
                        "filename": 1,
                        "mime_type": 1,
                        "filepath": 1,
                        "uploader_id": 1,
                        "uploaded_at": 1,
                    })
                    .build(),
            )
            .await?
            .ok_or(Error::msg("file not exists"))
    }

    async fn insert_uploaded_file(&self, file: UploadedFileCreate) -> Result<String, Error> {
        Ok(self
            .db
            .collection("uploaded_files")
            .insert_one(file, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or(Error::msg("return type of inserting is not ObjectId"))?
            .to_hex())
    }
}
