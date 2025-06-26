use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // User UUID
    pub email: String,      // User email
    pub exp: u64,          // Expiration time
    pub iat: u64,          // Issued at
    pub account_type: String, // User account type
}

#[derive(Debug)]
pub enum JwtError {
    TokenCreationError,
    TokenExpired,
    InvalidToken,
    MissingSecret,
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtError::TokenCreationError => write!(f, "Failed to create token"),
            JwtError::TokenExpired => write!(f, "Token has expired"),
            JwtError::InvalidToken => write!(f, "Invalid token"),
            JwtError::MissingSecret => write!(f, "JWT secret is missing"),
        }
    }
}

impl Claims {
    pub fn new(user_uuid: String, email: String, account_type: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Token expires in 24 hours
        let exp = now + (24 * 60 * 60);
        
        Claims {
            sub: user_uuid,
            email,
            exp,
            iat: now,
            account_type,
        }
    }
}

pub struct JwtService;

impl JwtService {
    fn get_secret() -> Result<String, JwtError> {
        env::var("JWT_SECRET").map_err(|_| JwtError::MissingSecret)
    }

    pub fn create_token(user_uuid: String, email: String, account_type: String) -> Result<String, JwtError> {
        let secret = Self::get_secret()?;
        let claims = Claims::new(user_uuid, email, account_type);
        
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref())
        )
        .map_err(|_| JwtError::TokenCreationError)
    }

    pub fn validate_token(token: &str) -> Result<Claims, JwtError> {
        let secret = Self::get_secret()?;
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default()
        )
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => JwtError::TokenExpired,
            _ => JwtError::InvalidToken,
        })?;

        Ok(token_data.claims)
    }
}