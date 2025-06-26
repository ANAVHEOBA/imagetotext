use serde::{Deserialize, Serialize};
use mongodb::bson::{DateTime, oid::ObjectId};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub uuid: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub account_type: AccountType,
    pub is_verified: bool,
    pub verification_code: Option<String>,
    pub verification_code_expires_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub last_login: Option<DateTime>,
    pub conversion_count: i32,
    pub plan: Plan,
    pub refresh_token: Option<String>,
    pub refresh_token_expires_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccountType {
    Individual,
    Student,
    Business,
    Enterprise,
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Plan {
    Free,
    Starter,
    Professional,
    Enterprise,
}

impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl User {
    pub fn new(
        email: String,
        password: String,
        full_name: String,
        account_type: AccountType,
    ) -> Result<Self, bcrypt::BcryptError> {
        let password_hash = hash(password, DEFAULT_COST)?;
        
        Ok(User {
            id: None,
            uuid: Uuid::new_v4().to_string(),
            email: email.to_lowercase(),
            password_hash,
            full_name,
            account_type,
            is_verified: false,
            verification_code: None,
            verification_code_expires_at: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            last_login: None,
            conversion_count: 0,
            plan: Plan::Free,
            refresh_token: None,
            refresh_token_expires_at: None,
        })
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, &self.password_hash)
    }

    pub fn can_convert(&self) -> bool {
        match self.plan {
            Plan::Free => self.conversion_count < 5,
            Plan::Starter => self.conversion_count < 50,
            Plan::Professional => self.conversion_count < 200,
            Plan::Enterprise => true, // Unlimited
        }
    }

    pub fn set_verification_code(&mut self, code: String) {
        self.verification_code = Some(code);
        let expires_at = DateTime::now().timestamp_millis() + (30 * 60 * 1000);
        self.verification_code_expires_at = Some(DateTime::from_millis(expires_at));
        self.updated_at = DateTime::now();
    }

    pub fn verify_code(&mut self, code: &str) -> bool {
        if let (Some(stored_code), Some(expires_at)) = (&self.verification_code, self.verification_code_expires_at) {
            if stored_code == code && expires_at > DateTime::now() {
                self.is_verified = true;
                self.verification_code = None;
                self.verification_code_expires_at = None;
                self.updated_at = DateTime::now();
                return true;
            }
        }
        false
    }

    pub fn set_refresh_token(&mut self, token: String) {
        self.refresh_token = Some(token);
        
        let expires_at = DateTime::now().timestamp_millis() + (30 * 24 * 60 * 60 * 1000);
        self.refresh_token_expires_at = Some(DateTime::from_millis(expires_at));
        self.updated_at = DateTime::now();
    }

    pub fn clear_refresh_token(&mut self) {
        self.refresh_token = None;
        self.refresh_token_expires_at = None;
        self.updated_at = DateTime::now();
    }

    pub fn validate_refresh_token(&self, token: &str) -> bool {
        if let (Some(stored_token), Some(expires_at)) = (&self.refresh_token, self.refresh_token_expires_at) {
            stored_token == token && expires_at > DateTime::now()
        } else {
            false
        }
    }
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Individual
    }
}

impl Default for Plan {
    fn default() -> Self {
        Plan::Free
    }
}