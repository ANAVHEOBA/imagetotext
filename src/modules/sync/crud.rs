use mongodb::{Collection, bson::{doc, oid::ObjectId}};
use futures::TryStreamExt;
use crate::modules::sync::model::Project;
use crate::modules::conversion::model::Conversion;
use mongodb::options::FindOptions;

pub struct SyncCRUD;

impl SyncCRUD {
    pub async fn create_project(
        user_id: &ObjectId,
        name: String,
        description: Option<String>,
        collection: &Collection<Project>
    ) -> mongodb::error::Result<Project> {
        let project = Project::new(user_id.clone(), name, description);
        let result = collection.insert_one(&project).await?;
        
        Ok(Project {
            id: result.inserted_id.as_object_id(),
            ..project
        })
    }

    pub async fn list_projects_paginated(
        user_id: &ObjectId,
        skip: i64,
        limit: i64,
        collection: &Collection<Project>
    ) -> mongodb::error::Result<Vec<Project>> {
        let filter = doc! {
            "user_id": user_id
        };
        
        let find_options = FindOptions::builder()
            .skip(skip as u64)
            .limit(limit as i64)
            .sort(doc! { "created_at": -1 })  // Sort by newest first
            .build();
        
        let mut cursor = collection.find(filter).with_options(find_options).await?;
        
        let mut projects = Vec::new();
        while let Some(project) = cursor.try_next().await? {
            projects.push(project);
        }
        
        Ok(projects)
    }

    pub async fn count_user_projects(
        user_id: &ObjectId,
        collection: &Collection<Project>
    ) -> mongodb::error::Result<i64> {
        let filter = doc! {
            "user_id": user_id
        };
        
        Ok(collection.count_documents(filter).await? as i64)
    }

    pub async fn find_project(
        project_id: &ObjectId,
        user_id: &ObjectId,
        collection: &Collection<Project>
    ) -> mongodb::error::Result<Option<Project>> {
        let filter = doc! {
            "_id": project_id,
            "user_id": user_id
        };
        collection.find_one(filter).await
    }

    pub async fn list_project_conversions(
        project_id: &ObjectId,
        user_id: &ObjectId,
        skip: i64,
        limit: i64,
        collection: &Collection<Conversion>
    ) -> mongodb::error::Result<Vec<Conversion>> {
        let filter = doc! {
            "user_id": user_id,
            "project_id": project_id
        };
        
        let find_options = FindOptions::builder()
            .skip(skip as u64)
            .limit(limit as i64)
            .sort(doc! { "created_at": -1 })
            .build();
        
        let mut cursor = collection.find(filter).with_options(find_options).await?;
        
        let mut conversions = Vec::new();
        while let Some(conversion) = cursor.try_next().await? {
            conversions.push(conversion);
        }
        
        Ok(conversions)
    }

    pub async fn list_unassigned_conversions(
        user_id: &ObjectId,
        skip: i64,
        limit: i64,
        collection: &Collection<Conversion>
    ) -> mongodb::error::Result<Vec<Conversion>> {
        let filter = doc! {
            "user_id": user_id,
            "project_id": null
        };
        
        let find_options = FindOptions::builder()
            .skip(skip as u64)
            .limit(limit as i64)
            .sort(doc! { "created_at": -1 })
            .build();
        
        let mut cursor = collection.find(filter).with_options(find_options).await?;
        
        let mut conversions = Vec::new();
        while let Some(conversion) = cursor.try_next().await? {
            conversions.push(conversion);
        }
        
        Ok(conversions)
    }

    pub async fn assign_conversions_to_project(
        project_id: &ObjectId,
        user_id: &ObjectId,
        job_ids: &[String],
        conversion_collection: &Collection<Conversion>,
        project_collection: &Collection<Project>
    ) -> mongodb::error::Result<()> {
        // Update all conversions
        let filter = doc! {
            "user_id": user_id,
            "job_id": { "$in": job_ids }
        };
        let update = doc! {
            "$set": {
                "project_id": project_id,
                "updated_at": mongodb::bson::DateTime::now()
            }
        };
        let result = conversion_collection.update_many(filter, update).await?;

        // Get total size of assigned conversions
        let size_filter = doc! {
            "user_id": user_id,
            "job_id": { "$in": job_ids }
        };
        let mut total_size: u64 = 0;
        let mut cursor = conversion_collection.find(size_filter).await?;
        while let Some(conversion) = cursor.try_next().await? {
            total_size += conversion.file_size;
        }

        // Update project statistics
        let project_filter = doc! {
            "_id": project_id,
            "user_id": user_id
        };
        let project_update = doc! {
            "$inc": {
                "conversion_count": result.modified_count as i32,
                "total_storage_bytes": total_size as i64
            },
            "$set": {
                "updated_at": mongodb::bson::DateTime::now()
            }
        };
        project_collection.update_one(project_filter, project_update).await?;

        Ok(())
    }
}
