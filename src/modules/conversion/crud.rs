use mongodb::{Collection, bson::{doc, oid::ObjectId}};
use crate::modules::conversion::model::Conversion;

pub struct ConversionCRUD;

impl ConversionCRUD {
    pub async fn create(conversion: &Conversion, collection: &Collection<Conversion>) -> mongodb::error::Result<ObjectId> {
        let insert_result = collection.insert_one(conversion).await?;
        Ok(insert_result.inserted_id.as_object_id().unwrap())
    }

    pub async fn find_by_job_id(job_id: &str, collection: &Collection<Conversion>) -> mongodb::error::Result<Option<Conversion>> {
        collection.find_one(doc! { "job_id": job_id }).await
    }
} 