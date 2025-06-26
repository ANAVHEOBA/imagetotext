use serde::{Deserialize, Serialize};
use mongodb::bson::{DateTime, oid::ObjectId};
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversion {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub job_id: String,
    pub user_id: ObjectId,
    pub original_filename: String,
    pub cloudinary_url: String,
    pub cloudinary_public_id: String,
    pub file_size: u64,
    pub mime_type: String,
    pub status: ConversionStatus,
    pub extracted_text: Option<String>,
    pub processing_time_ms: Option<u64>,
    pub error_message: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub completed_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConversionStatus {
    Processing,
    Completed,
    Failed,
}

impl fmt::Display for ConversionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionStatus::Processing => write!(f, "processing"),
            ConversionStatus::Completed => write!(f, "completed"),
            ConversionStatus::Failed => write!(f, "failed"),
        }
    }
}

impl Conversion {
    pub fn new(
        user_id: ObjectId,
        original_filename: String,
        file_size: u64,
        mime_type: String,
    ) -> Self {
        Conversion {
            id: None,
            job_id: Uuid::new_v4().to_string(),
            user_id,
            original_filename,
            cloudinary_url: String::new(),
            cloudinary_public_id: String::new(),
            file_size,
            mime_type,
            status: ConversionStatus::Processing,
            extracted_text: None,
            processing_time_ms: None,
            error_message: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            completed_at: None,
        }
    }

    pub fn mark_completed(
        &mut self, 
        extracted_text: String, 
        processing_time_ms: u64, 
        cloudinary_url: String,
        cloudinary_public_id: String,
    ) {
        self.status = ConversionStatus::Completed;
        self.extracted_text = Some(extracted_text);
        self.processing_time_ms = Some(processing_time_ms);
        self.cloudinary_url = cloudinary_url;
        self.cloudinary_public_id = cloudinary_public_id;
        self.updated_at = DateTime::now();
        self.completed_at = Some(DateTime::now());
    }
}

impl Default for ConversionStatus {
    fn default() -> Self {
        ConversionStatus::Processing
    }
} 