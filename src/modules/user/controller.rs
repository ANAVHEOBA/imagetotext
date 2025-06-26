use actix_web::{web, HttpResponse, Result, Error};
use validator::Validate;
use mongodb::bson::{doc, DateTime};
use uuid::Uuid;

use crate::modules::user::crud::{UserCRUD, UserError, authenticate_user, create_user_account};
use crate::modules::user::schema::{
    RegisterRequest, LoginRequest, AuthResponse, UserResponse, 
    ErrorResponse, SuccessResponse, ValidationErrorResponse, VerificationRequest,
    RefreshTokenRequest, TokenResponse
};
use crate::modules::user::model::Plan;
use crate::services::jwt::JwtService;
use crate::services::email::EmailService;
use crate::config::environment::Config;
use crate::config::database::get_users_collection;

pub struct UserController;

impl UserController {
    /// Register a new user account
    pub async fn register(register_data: web::Json<RegisterRequest>) -> Result<HttpResponse, Error> {
        println!("[Register] - Attempting to register new user...");
        // Validate the request data
        if let Err(validation_errors) = register_data.validate() {
            let field_errors: std::collections::HashMap<String, Vec<String>> = validation_errors
                .field_errors()
                .iter()
                .map(|(field, errors)| {
                    let error_messages: Vec<String> = errors
                        .iter()
                        .map(|e| e.message.clone().unwrap_or_else(|| "Invalid field".into()).to_string())
                        .collect();
                    (field.to_string(), error_messages)
                })
                .collect();

            let error_response = ValidationErrorResponse {
                error: "Validation failed".to_string(),
                field_errors,
            };

            return Ok(HttpResponse::BadRequest().json(error_response));
        }

        println!("[Register] - Validation successful. Creating user account...");
        // Create the user account
        match create_user_account(register_data.into_inner()).await {
            Ok(mut user) => {
                println!("[Register] - User account created successfully in DB. User UUID: {}", user.uuid);
                // Generate verification code and update user
                let verification_code = EmailService::generate_verification_code();
                user.set_verification_code(verification_code.clone());

                // Update user in database with verification code
                if let Some(user_id) = user.id {
                    let collection = get_users_collection().await
                        .map_err(actix_web::error::ErrorInternalServerError)?;

                    collection.update_one(
                        doc! { "_id": user_id },
                        doc! { 
                            "$set": { 
                                "verification_code": &verification_code,
                                "verification_code_expires_at": user.verification_code_expires_at,
                                "updated_at": user.updated_at
                            }
                        },
                    ).await.map_err(actix_web::error::ErrorInternalServerError)?;
                }

                println!("[Register] - Database updated with verification code.");

                // Send verification email
                let config = Config::new();
                let email_service = EmailService::new(config);
                println!("[Register] - Sending verification email to {}...", &user.email);
                if let Err(e) = email_service.send_verification_email(&user.email, &verification_code).await {
                    println!("[Register] - ERROR: Failed to send verification email: {}", e);
                    // Continue with registration even if email fails
                } else {
                    println!("[Register] - Verification email sent successfully.");
                }

                // Generate JWT token
                println!("[Register] - Generating JWT token...");
                let token = JwtService::create_token(
                    user.uuid.clone(),
                    user.email.clone(),
                    user.account_type.to_string(),
                ).map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!("Token generation failed: {}", e))
                })?;

                // Create user response
                let user_response = UserResponse {
                    uuid: user.uuid,
                    email: user.email,
                    full_name: user.full_name,
                    account_type: user.account_type.to_string(),
                    is_verified: user.is_verified,
                    conversion_count: user.conversion_count,
                    plan: user.plan.to_string(),
                    created_at: user.created_at.to_string(),
                };

                let auth_response = AuthResponse {
                    user: user_response,
                    token,
                    message: "Account created successfully. Please check your email for verification code.".to_string(),
                };

                Ok(HttpResponse::Created().json(auth_response))
            }
            Err(UserError::UserAlreadyExists) => {
                Ok(HttpResponse::Conflict().json(ErrorResponse {
                    error: "User already exists".to_string(),
                    message: "An account with this email already exists".to_string(),
                }))
            }
            Err(UserError::PasswordError) => {
                Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Password error".to_string(),
                    message: "Failed to process password".to_string(),
                }))
            }
            Err(UserError::DatabaseError) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    message: "Failed to create account. Please try again.".to_string(),
                }))
            }
        }
    }

    /// Login user
    pub async fn login(login_data: web::Json<LoginRequest>) -> Result<HttpResponse, Error> {
        let email = login_data.email.trim();
        let password = &login_data.password;

        // Basic validation
        if email.is_empty() || password.is_empty() {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid input".to_string(),
                message: "Email and password are required".to_string(),
            }));
        }

        // Authenticate user
        match authenticate_user(email, password).await {
            Ok(Some(mut user)) => {
                // Check if the user's email is verified
                if !user.is_verified {
                    return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                        error: "Email not verified".to_string(),
                        message: "Please verify your email before logging in.".to_string(),
                    }));
                }

                // Generate JWT token
                let access_token = JwtService::create_token(
                    user.uuid.clone(),
                    user.email.clone(),
                    user.account_type.to_string(),
                ).map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!("Token generation failed: {}", e))
                })?;

                // Generate refresh token
                let refresh_token = Uuid::new_v4().to_string();
                
                // Update refresh token in database
                if let Err(_) = UserCRUD::update_refresh_token(&user.uuid, Some(refresh_token.clone())).await {
                    return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Database error".to_string(),
                        message: "Failed to generate refresh token.".to_string(),
                    }));
                }

                // Create user response
                let user_response = UserResponse {
                    uuid: user.uuid,
                    email: user.email,
                    full_name: user.full_name,
                    account_type: user.account_type.to_string(),
                    is_verified: user.is_verified,
                    conversion_count: user.conversion_count,
                    plan: user.plan.to_string(),
                    created_at: user.created_at.to_string(),
                };

                let auth_response = AuthResponse {
                    user: user_response,
                    token: access_token.clone(),
                    message: "Login successful".to_string(),
                };

                // Return both tokens
                Ok(HttpResponse::Ok()
                    .append_header(("X-Refresh-Token", refresh_token))
                    .json(auth_response))
            }
            Ok(None) => {
                Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "Authentication failed".to_string(),
                    message: "Invalid email or password".to_string(),
                }))
            }
            Err(UserError::DatabaseError) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    message: "Failed to authenticate. Please try again.".to_string(),
                }))
            }
            Err(_) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Unknown error".to_string(),
                    message: "An unexpected error occurred".to_string(),
                }))
            }
        }
    }

    /// Get current user profile
    pub async fn get_profile(user_uuid: web::Path<String>) -> Result<HttpResponse> {
        match UserCRUD::find_by_uuid(&user_uuid).await {
            Ok(Some(user)) => {
                let user_response = UserResponse {
                    uuid: user.uuid,
                    email: user.email,
                    full_name: user.full_name,
                    account_type: user.account_type.to_string(),
                    is_verified: user.is_verified,
                    conversion_count: user.conversion_count,
                    plan: user.plan.to_string(),
                    created_at: user.created_at.to_string(),
                };

                Ok(HttpResponse::Ok().json(user_response))
            }
            Ok(None) => {
                Ok(HttpResponse::NotFound().json(ErrorResponse {
                    error: "User not found".to_string(),
                    message: "User with this UUID does not exist".to_string(),
                }))
            }
            Err(UserError::DatabaseError) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    message: "Failed to fetch user profile".to_string(),
                }))
            }
            Err(_) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Unknown error".to_string(),
                    message: "An unexpected error occurred".to_string(),
                }))
            }
        }
    }

    /// Get user statistics (admin endpoint)
    pub async fn get_stats() -> Result<HttpResponse> {
        match UserCRUD::get_user_stats().await {
            Ok(stats) => {
                Ok(HttpResponse::Ok().json(SuccessResponse {
                    message: "Statistics retrieved successfully".to_string(),
                    data: Some(serde_json::to_value(stats).unwrap()),
                }))
            }
            Err(UserError::DatabaseError) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    message: "Failed to fetch statistics".to_string(),
                }))
            }
            Err(_) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Unknown error".to_string(),
                    message: "An unexpected error occurred".to_string(),
                }))
            }
        }
    }

    /// Check if user can perform conversion
    pub async fn check_conversion_limit(user_uuid: web::Path<String>) -> Result<HttpResponse> {
        match UserCRUD::find_by_uuid(&user_uuid).await {
            Ok(Some(user)) => {
                let can_convert = user.can_convert();
                let response = serde_json::json!({
                    "can_convert": can_convert,
                    "conversion_count": user.conversion_count,
                    "plan": user.plan.to_string(),
                    "limit": match user.plan {
                        Plan::Free => 5,
                        Plan::Starter => 50,
                        Plan::Professional => 200,
                        Plan::Enterprise => -1, // Unlimited
                    }
                });

                Ok(HttpResponse::Ok().json(SuccessResponse {
                    message: "Conversion limit checked".to_string(),
                    data: Some(response),
                }))
            }
            Ok(None) => {
                Ok(HttpResponse::NotFound().json(ErrorResponse {
                    error: "User not found".to_string(),
                    message: "User with this UUID does not exist".to_string(),
                }))
            }
            Err(_) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    message: "Failed to check conversion limit".to_string(),
                }))
            }
        }
    }

    /// Verify email with verification code
    pub async fn verify_email(
        user_uuid: web::Path<String>,
        verification_data: web::Json<VerificationRequest>,
    ) -> Result<HttpResponse, Error> {
        let uuid_str = user_uuid.into_inner();
        match UserCRUD::find_by_uuid(&uuid_str).await {
            Ok(Some(mut user)) => {
                if user.is_verified {
                    return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        error: "Already verified".to_string(),
                        message: "Email is already verified".to_string(),
                    }));
                }

                if user.verify_code(&verification_data.code) {
                    // Update user in database
                    let collection = get_users_collection().await
                        .map_err(actix_web::error::ErrorInternalServerError)?;

                    collection.update_one(
                        doc! { "uuid": uuid_str },
                        doc! { 
                            "$set": { 
                                "is_verified": true,
                                "verification_code": None::<String>,
                                "verification_code_expires_at": None::<DateTime>,
                                "updated_at": DateTime::now()
                            }
                        },
                    ).await.map_err(actix_web::error::ErrorInternalServerError)?;

                    Ok(HttpResponse::Ok().json(SuccessResponse {
                        message: "Email verified successfully".to_string(),
                        data: None,
                    }))
                } else {
                    Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        error: "Invalid code".to_string(),
                        message: "Invalid or expired verification code".to_string(),
                    }))
                }
            }
            Ok(None) => {
                Ok(HttpResponse::NotFound().json(ErrorResponse {
                    error: "User not found".to_string(),
                    message: "User with this UUID does not exist".to_string(),
                }))
            }
            Err(_) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    message: "Failed to verify email".to_string(),
                }))
            }
        }
    }

    /// Logout user by invalidating refresh token
    pub async fn logout(user_uuid: web::Path<String>) -> Result<HttpResponse> {
        match UserCRUD::update_refresh_token(&user_uuid, None).await {
            Ok(_) => {
                Ok(HttpResponse::Ok().json(SuccessResponse {
                    message: "Successfully logged out".to_string(),
                    data: None,
                }))
            }
            Err(_) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    message: "Failed to logout. Please try again.".to_string(),
                }))
            }
        }
    }

    /// Refresh access token using refresh token
    pub async fn refresh_token(refresh_request: web::Json<RefreshTokenRequest>) -> Result<HttpResponse> {
        match UserCRUD::find_by_refresh_token(&refresh_request.refresh_token).await {
            Ok(Some(user)) => {
                // Generate new access token
                let access_token = JwtService::create_token(
                    user.uuid.clone(),
                    user.email.clone(),
                    user.account_type.to_string(),
                ).map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!("Token generation failed: {}", e))
                })?;

                // Generate new refresh token
                let new_refresh_token = Uuid::new_v4().to_string();
                
                // Update refresh token in database
                UserCRUD::update_refresh_token(&user.uuid, Some(new_refresh_token.clone())).await
                    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update refresh token"))?;

                Ok(HttpResponse::Ok().json(TokenResponse {
                    access_token,
                    refresh_token: new_refresh_token,
                    token_type: "Bearer".to_string(),
                    expires_in: 3600, // 1 hour
                }))
            }
            Ok(None) => {
                Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "Invalid token".to_string(),
                    message: "Refresh token is invalid or expired".to_string(),
                }))
            }
            Err(_) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    message: "Failed to refresh token. Please try again.".to_string(),
                }))
            }
        }
    }
}