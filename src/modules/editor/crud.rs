use mongodb::{Collection, bson::{doc, oid::ObjectId}, options::FindOptions};
use crate::modules::editor::model::{Preview, PreviewStatus};
use chrono::Utc;

pub struct EditorCRUD;

impl EditorCRUD {
    pub async fn create_preview(
        user_id: &ObjectId,
        conversion_id: &str,
        html_content: &str,
        original_filename: &str,
        collection: &Collection<Preview>,
    ) -> mongodb::error::Result<Preview> {
        let now = mongodb::bson::DateTime::from_millis(Utc::now().timestamp_millis());
        
        let preview = Preview {
            id: None,
            conversion_id: conversion_id.to_string(),
            user_id: user_id.clone(),
            html_content: html_content.to_string(),
            original_filename: original_filename.to_string(),
            preview_status: PreviewStatus::Ready,
            total_pages: 0, // Will be updated after processing
            word_count: 0,  // Will be updated after processing
            format: crate::modules::editor::model::DocumentFormat::Docx,
            created_at: now,
            updated_at: now,
        };

        let insert_result = collection.insert_one(&preview).await?;
        let id = insert_result.inserted_id.as_object_id().unwrap();
        
        Ok(Preview { id: Some(id), ..preview })
    }

    pub async fn get_preview(
        user_id: &ObjectId,
        conversion_id: &str,
        collection: &Collection<Preview>,
    ) -> mongodb::error::Result<Option<Preview>> {
        collection
            .find_one(doc! {
                "user_id": user_id,
                "conversion_id": conversion_id
            })
            .await
    }

    pub async fn list_previews(
        user_id: &ObjectId,
        skip: i64,
        limit: i64,
        collection: &Collection<Preview>,
    ) -> mongodb::error::Result<Vec<Preview>> {
        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .skip(Some(skip as u64))
            .limit(Some(limit))
            .build();

        let mut cursor = collection
            .find(doc! { "user_id": user_id })
            .with_options(options)
            .await?;

        let mut previews = Vec::new();
        while cursor.advance().await? {
            previews.push(cursor.deserialize_current()?);
        }
        Ok(previews)
    }

    pub async fn count_user_previews(
        user_id: &ObjectId,
        collection: &Collection<Preview>,
    ) -> mongodb::error::Result<i64> {
        let count = collection
            .count_documents(doc! { "user_id": user_id })
            .await?;
        Ok(count as i64)
    }

    #[allow(dead_code)]
    pub async fn update_preview_status(
        user_id: &ObjectId,
        conversion_id: &str,
        status: PreviewStatus,
        collection: &Collection<Preview>,
    ) -> mongodb::error::Result<bool> {
        let status_str = status.to_string();
        let result = collection
            .update_one(
                doc! {
                    "user_id": user_id,
                    "conversion_id": conversion_id
                },
                doc! {
                    "$set": {
                        "preview_status": status_str,
                        "updated_at": mongodb::bson::DateTime::from_millis(Utc::now().timestamp_millis())
                    }
                },
            )
            .await?;
        
        Ok(result.modified_count > 0)
    }

    #[allow(dead_code)]
    pub async fn delete_preview(
        user_id: &ObjectId,
        conversion_id: &str,
        collection: &Collection<Preview>,
    ) -> mongodb::error::Result<bool> {
        let result = collection
            .delete_one(doc! {
                "user_id": user_id,
                "conversion_id": conversion_id
            })
            .await?;
        
        Ok(result.deleted_count > 0)
    }
}
