use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub name: String,
    pub description: Option<String>,
    pub cloudinary_folder: String,  // Cloudinary folder path
    pub conversion_count: i32,
    pub total_storage_bytes: u64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Project {
    pub fn new(user_id: ObjectId, name: String, description: Option<String>) -> Self {
        let now = DateTime::now();
        let cloudinary_folder = format!("projects/{}/{}", user_id.to_hex(), name.to_lowercase().replace(" ", "_"));
        
        Self {
            id: None,
            user_id,
            name,
            description,
            cloudinary_folder,
            conversion_count: 0,
            total_storage_bytes: 0,
            created_at: now,
            updated_at: now,
        }
    }
}

