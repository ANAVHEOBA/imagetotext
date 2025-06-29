use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssignConversionsRequest {
    pub conversion_ids: Vec<String>,  // job_ids of conversions to assign
}

#[derive(Debug, Serialize)]
pub struct ProjectListResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cloudinary_folder: String,
    pub conversion_count: i32,
    pub total_storage_bytes: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectListResult {
    pub projects: Vec<ProjectListResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct ProjectConversionsResponse {
    pub conversions: Vec<ConversionListItem>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct ConversionListItem {
    pub job_id: String,
    pub original_filename: String,
    pub cloudinary_url: String,
    pub file_size: u64,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub message: String,
    pub data: Option<serde_json::Value>,
}
