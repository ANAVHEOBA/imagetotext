use actix_web::{web, Scope};
use crate::modules::user::controller::UserController;
use crate::middleware::user_auth::{Authentication, RequireVerifiedEmail};

pub fn user_routes() -> Scope {
    web::scope("/auth")
        // Public routes (no authentication required)
        .service(
            web::scope("/public")
                .route("/register", web::post().to(UserController::register))
                .route("/verify/{user_uuid}", web::post().to(UserController::verify_email))
                .route("/login", web::post().to(UserController::login))
                .route("/refresh-token", web::post().to(UserController::refresh_token))
        )
        // Protected routes (require authentication)
        .service(
            web::scope("/protected")
                .wrap(Authentication::new())  // First check if user is authenticated
                .service(
                    // Routes that require both authentication and email verification
                    web::scope("/verified")
                        .wrap(RequireVerifiedEmail::new())  // Then check if email is verified
                        .route("/profile/{user_uuid}", web::get().to(UserController::get_profile))
                        .route("/conversion-limit/{user_uuid}", web::get().to(UserController::check_conversion_limit))
                )
                // Routes that only require authentication (no email verification needed)
                .route("/stats", web::get().to(UserController::get_stats))
                .route("/logout/{user_uuid}", web::post().to(UserController::logout))
        )
}

// Helper function to mount all routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(user_routes());
}

/*
Route Structure:

1. Public Routes (No Auth Required):
   - POST /api/auth/public/register
   - POST /api/auth/public/verify/{user_uuid}
   - POST /api/auth/public/login

2. Protected Routes (Auth Required):
   A. Email Verification Required:
      - GET /api/auth/protected/verified/profile/{user_uuid}
      - GET /api/auth/protected/verified/conversion-limit/{user_uuid}
   
   B. Only Authentication Required:
      - GET /api/auth/protected/stats

Security Flow:
1. Authentication Middleware checks:
   - Valid JWT token in Authorization header
   - Token not expired
   - Token format is correct

2. Email Verification Middleware checks:
   - User exists in database
   - User's email is verified
   - Only applies to routes under /verified/

Error Responses:
- 401 Unauthorized: Missing/invalid token
- 403 Forbidden: Email not verified
- 404 Not Found: User not found
*/
