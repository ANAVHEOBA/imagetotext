use std::env;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Config {
    pub jwt_secret: String,
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub port: String,
    pub environment: String,
    pub cloudinary_api_key: String,
    pub cloudinary_cloud_name: String,
    pub cloudinary_api_secret: String,
    pub email_user: String,
    pub email_password: String,
    pub open_router_api_key: String,
    pub frontend_url: String,
}

impl Config {
    pub fn new() -> Self {
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string());
        let mongodb_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        let mongodb_database = env::var("MONGODB_DATABASE").unwrap_or_else(|_| "imagetotext".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:4200".to_string());

        let cloudinary_api_key = env::var("CLOUDINARY_API_KEY").unwrap_or_else(|_| "your_cloudinary_api_key".to_string());
        let cloudinary_cloud_name = env::var("CLOUDINARY_CLOUD_NAME").unwrap_or_else(|_| "your_cloudinary_cloud_name".to_string());
        let cloudinary_api_secret = env::var("CLOUDINARY_API_SECRET").unwrap_or_else(|_| "your_cloudinary_api_secret".to_string());
        let email_user = env::var("EMAIL_USER").unwrap_or_else(|_| "your_email_user".to_string());
        let email_password = env::var("EMAIL_PASSWORD").unwrap_or_else(|_| "your_email_password".to_string());
        let open_router_api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| "your_open_router_api_key".to_string());

        Config {
            jwt_secret,
            mongodb_uri,
            mongodb_database,
            port,
            environment,
            cloudinary_api_key,
            cloudinary_cloud_name,
            cloudinary_api_secret,
            email_user,
            email_password,
            open_router_api_key,
            frontend_url,
        }
    }
    #[allow(dead_code)]
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
    #[allow(dead_code)]
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}