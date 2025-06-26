use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    #[validate(length(min = 2, max = 100, message = "Full name must be between 2 and 100 characters"))]
    pub full_name: String,
    
    #[serde(default)]
    pub account_type: AccountTypeRequest,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerificationRequest {
    #[validate(length(equal = 6, message = "Verification code must be 6 digits"))]
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub uuid: String,
    pub email: String,
    pub full_name: String,
    pub account_type: String,
    pub is_verified: bool,
    pub conversion_count: i32,
    pub plan: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum AccountTypeRequest {
    Individual,
    Student,
    Business,
    Enterprise,
}

impl Default for AccountTypeRequest {
    fn default() -> Self {
        AccountTypeRequest::Individual
    }
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

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}