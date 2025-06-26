use mongodb::bson::{doc, oid::ObjectId, DateTime};
use mongodb::error::Error as MongoError;

use crate::modules::user::model::{User, AccountType};
use crate::modules::user::schema::{RegisterRequest, AccountTypeRequest};
use crate::config::database::get_users_collection;

#[derive(Debug)]
pub enum UserError {
    DatabaseError,
    UserAlreadyExists,
    PasswordError,
}

impl From<MongoError> for UserError {
    fn from(_: MongoError) -> Self {
        UserError::DatabaseError
    }
}

impl From<bcrypt::BcryptError> for UserError {
    fn from(_: bcrypt::BcryptError) -> Self {
        UserError::PasswordError
    }
}

pub struct UserCRUD;

impl UserCRUD {
    /// Create a new user account
    pub async fn create_user(register_data: RegisterRequest) -> Result<User, UserError> {
        println!("[CRUD] - Getting database collection...");
        let collection = get_users_collection().await
            .map_err(|_| UserError::DatabaseError)?;

        // Check if user already exists
        println!("[CRUD] - Checking if user exists for email: {}", &register_data.email);
        let existing_user = collection
            .find_one(doc! { "email": register_data.email.to_lowercase() })
            .await?;
        println!("[CRUD] - User check complete.");

        if existing_user.is_some() {
            println!("[CRUD] - User already exists.");
            return Err(UserError::UserAlreadyExists);
        }
        println!("[CRUD] - User does not exist. Proceeding with creation...");

        // Convert AccountTypeRequest to AccountType
        let account_type = match register_data.account_type {
            AccountTypeRequest::Individual => AccountType::Individual,
            AccountTypeRequest::Student => AccountType::Student,
            AccountTypeRequest::Business => AccountType::Business,
            AccountTypeRequest::Enterprise => AccountType::Enterprise,
        };

        // Create new user
        println!("[CRUD] - Hashing password and creating new User struct...");
        let user = User::new(
            register_data.email,
            register_data.password,
            register_data.full_name,
            account_type,
        )?;
        println!("[CRUD] - New User struct created.");

        // Insert user into database
        println!("[CRUD] - Inserting new user into database...");
        let result = collection
            .insert_one(&user)
            .await?;
        println!("[CRUD] - User inserted successfully.");

        // Get the inserted user with the generated ObjectId
        let mut created_user = user;
        created_user.id = Some(result.inserted_id.as_object_id().unwrap());

        Ok(created_user)
    }

    /// Find user by email
    pub async fn find_by_email(email: &str) -> Result<Option<User>, UserError> {
        let collection = get_users_collection().await
            .map_err(|_| UserError::DatabaseError)?;

        let user = collection
            .find_one(doc! { "email": email.to_lowercase() })
            .await?;

        Ok(user)
    }

    /// Find user by UUID
    pub async fn find_by_uuid(uuid: &str) -> Result<Option<User>, UserError> {
        let collection = get_users_collection().await
            .map_err(|_| UserError::DatabaseError)?;

        let user = collection
            .find_one(doc! { "uuid": uuid })
            .await?;

        Ok(user)
    }

    /// Update user's last login time
    pub async fn update_last_login(user_id: ObjectId) -> Result<(), UserError> {
        let collection = get_users_collection().await
            .map_err(|_| UserError::DatabaseError)?;

        collection
            .update_one(
                doc! { "_id": user_id },
                doc! { 
                    "$set": { 
                        "last_login": DateTime::now(),
                        "updated_at": DateTime::now()
                    }
                },
            )
            .await?;

        Ok(())
    }

    /// Increment user's conversion count by one
    pub async fn increment_conversion_count(user_uuid: &str) -> Result<(), UserError> {
        let collection = get_users_collection().await
            .map_err(|_| UserError::DatabaseError)?;

        collection
            .update_one(
                doc! { "uuid": user_uuid },
                doc! {
                    "$inc": { "conversion_count": 1 },
                    "$set": { "updated_at": DateTime::now() }
                },
            )
            .await?;

        Ok(())
    }

    /// Get user statistics
    pub async fn get_user_stats() -> Result<UserStats, UserError> {
        let collection = get_users_collection().await
            .map_err(|_| UserError::DatabaseError)?;

        let total_users = collection.count_documents(doc! {}).await?;
        let verified_users = collection.count_documents(doc! { "is_verified": true }).await?;
        
        // Calculate 24 hours ago in milliseconds
        let twenty_four_hours_ago = DateTime::now().timestamp_millis() - (24 * 60 * 60 * 1000);
        let today_users = collection.count_documents(
            doc! { 
                "created_at": { 
                    "$gte": DateTime::from_millis(twenty_four_hours_ago)
                }
            }, 
        ).await?;

        Ok(UserStats {
            total_users: total_users as i32,
            verified_users: verified_users as i32,
            today_users: today_users as i32,
        })
    }

    pub async fn update_refresh_token(user_uuid: &str, refresh_token: Option<String>) -> Result<(), UserError> {
        let collection = get_users_collection().await
            .map_err(|_| UserError::DatabaseError)?;

        let expires_at = refresh_token.as_ref().map(|_| {
            DateTime::from_millis(DateTime::now().timestamp_millis() + (30 * 24 * 60 * 60 * 1000))
        });

        let update = match refresh_token {
            Some(token) => doc! {
                "$set": {
                    "refresh_token": token,
                    "refresh_token_expires_at": expires_at,
                    "updated_at": DateTime::now()
                }
            },
            None => doc! {
                "$set": {
                    "refresh_token": None::<String>,
                    "refresh_token_expires_at": None::<DateTime>,
                    "updated_at": DateTime::now()
                }
            },
        };

        collection
            .update_one(doc! { "uuid": user_uuid }, update)
            .await?;

        Ok(())
    }

    pub async fn find_by_refresh_token(refresh_token: &str) -> Result<Option<User>, UserError> {
        let collection = get_users_collection().await
            .map_err(|_| UserError::DatabaseError)?;

        let user = collection
            .find_one(doc! { 
                "refresh_token": refresh_token,
                "refresh_token_expires_at": { "$gt": DateTime::now() }
            })
            .await?;

        Ok(user)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UserStats {
    pub total_users: i32,
    pub verified_users: i32,
    pub today_users: i32,
}

// Helper functions for common operations
pub async fn create_user_account(register_data: RegisterRequest) -> Result<User, UserError> {
    UserCRUD::create_user(register_data).await
}

pub async fn authenticate_user(email: &str, password: &str) -> Result<Option<User>, UserError> {
    if let Some(user) = UserCRUD::find_by_email(email).await? {
        if user.verify_password(password)? {
            // Update last login
            if let Some(user_id) = user.id {
                UserCRUD::update_last_login(user_id).await?;
            }
            Ok(Some(user))
        } else {
            Ok(None) // Invalid password
        }
    } else {
        Ok(None) // User not found
    }
}