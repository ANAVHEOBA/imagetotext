use serde::Serialize;
use mongodb::bson::DateTime;

#[derive(Debug, Serialize)]
pub struct PreviewResponse {
    pub html_content: String,
    pub metadata: DocumentMetadata,
    pub conversion_id: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct DocumentMetadata {
    pub total_pages: i32,
    pub word_count: i32,
    pub last_modified: DateTime,
    pub format: String,  // "docx", "pdf", "odt"
    pub file_size: Option<u64>,  // Size in bytes
    pub download_filename: Option<String>,  // Suggested download filename
}

#[derive(Debug, Serialize)]
pub struct PreviewListResponse {
    pub previews: Vec<PreviewSummary>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct PreviewSummary {
    pub conversion_id: String,
    pub original_filename: String,
    pub preview_status: String,
    pub created_at: DateTime,
    pub last_modified: DateTime,
    pub file_size: Option<u64>,  // Size in bytes
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct DownloadMetadata {
    pub filename: String,
    pub file_size: u64,
    pub mime_type: String,
    pub format: String,
}

#[derive(Debug, Serialize)]
pub struct DownloadResponse {
    pub metadata: DownloadMetadata,
    pub download_url: String,
}
