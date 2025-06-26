use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub job_id: String,
    pub message: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ConversionStatusResponse {
    pub job_id: String,
    pub status: String,
    pub extracted_text: Option<String>,
    pub confidence_score: Option<f64>,
    pub processing_time_ms: Option<u64>,
    pub error_message: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConversionHistoryResponse {
    pub conversions: Vec<ConversionListItem>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct ConversionListItem {
    pub job_id: String,
    pub original_filename: String,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub file_size: u64,
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

// Validation error response
#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    pub error: String,
    pub field_errors: std::collections::HashMap<String, Vec<String>>,
}

// File upload constraints
pub const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024; // 5 MB
pub const ALLOWED_MIME_TYPES: [&str; 4] = [
    "image/jpeg",
    "image/jpg", 
    "image/png",
    "image/gif"
];

pub fn is_allowed_mime_type(mime: &str) -> bool {
    matches!(mime, "image/jpeg" | "image/png" | "image/gif")
}

#[derive(Serialize)]
pub struct ConversionResultResponse {
    pub job_id: String,
    pub status: String,
    pub original_filename: String,
    pub extracted_text: String,
    pub created_at: String,
    pub processing_time_ms: u64,
} 