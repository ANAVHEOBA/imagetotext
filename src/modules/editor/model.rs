use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Preview {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub conversion_id: String,
    pub user_id: ObjectId,
    pub html_content: String,
    pub original_filename: String,
    pub preview_status: PreviewStatus,
    pub total_pages: i32,
    pub word_count: i32,
    pub format: DocumentFormat,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PreviewStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "generating")]
    Generating,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "error")]
    Error,
}

impl fmt::Display for PreviewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreviewStatus::Pending => write!(f, "pending"),
            PreviewStatus::Generating => write!(f, "generating"),
            PreviewStatus::Ready => write!(f, "ready"),
            PreviewStatus::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentFormat {
    #[serde(rename = "docx")]
    Docx,
    #[serde(rename = "pdf")]
    PDF,
    #[serde(rename = "odt")]
    ODT,
}

impl fmt::Display for DocumentFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentFormat::Docx => write!(f, "docx"),
            DocumentFormat::PDF => write!(f, "pdf"),
            DocumentFormat::ODT => write!(f, "odt"),
        }
    }
}
