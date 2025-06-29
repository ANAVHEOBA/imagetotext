use mongodb::{Client, Collection, Database, IndexModel, options::IndexOptions};
use mongodb::bson::doc;
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::modules::user::model::User;
use crate::modules::conversion::model::Conversion;
use crate::modules::sync::model::Project;
use crate::modules::editor::model::Preview;
use crate::config::environment::Config;

static DB_INSTANCE: OnceCell<Arc<Database>> = OnceCell::const_new();

async fn get_db() -> Arc<Database> {
    DB_INSTANCE.get_or_init(|| async {
        let config = Config::new();
        let client = Client::with_uri_str(&config.mongodb_uri)
            .await
            .expect("Failed to initialize MongoDB client");
        
        let db = client.database(&config.mongodb_database);
        
        // Create indices
        let user_collection: Collection<User> = db.collection("users");
        let email_index = IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();
        user_collection.create_index(email_index)
            .await
            .expect("Failed to create email index");

        let conversion_collection: Collection<Conversion> = db.collection("conversions");
        let job_id_index = IndexModel::builder()
            .keys(doc! { "job_id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();
        conversion_collection.create_index(job_id_index)
            .await
            .expect("Failed to create job_id index");

        let user_id_index = IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .build();
        conversion_collection.create_index(user_id_index)
            .await
            .expect("Failed to create user_id index");

        // Create indices for projects collection
        let projects_collection: Collection<Project> = db.collection("projects");
        let project_user_id_index = IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .build();
        projects_collection.create_index(project_user_id_index)
            .await
            .expect("Failed to create project user_id index");

        Arc::new(db)
    })
    .await
    .clone()
}

pub async fn get_users_collection() -> mongodb::error::Result<Collection<User>> {
    let db = get_db().await;
    Ok(db.collection("users"))
}

pub async fn get_conversion_collection() -> mongodb::error::Result<Collection<Conversion>> {
    let db = get_db().await;
    Ok(db.collection("conversions"))
}

pub async fn get_projects_collection() -> mongodb::error::Result<Collection<Project>> {
    let db = get_db().await;
    Ok(db.collection("projects"))
}

pub async fn get_previews_collection() -> mongodb::error::Result<Collection<Preview>> {
    let db = get_db().await;
    Ok(db.collection("previews"))
}