use actix_web::web;
use crate::modules::conversion::controller::ConversionController;
use crate::middleware::user_auth::Authentication;

pub fn configure_conversion_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/conversion")
            .wrap(Authentication::new())  // Require authentication for all conversion routes
            .route("/upload", web::post().to(ConversionController::upload_image))
            .route("/{job_id}", web::get().to(ConversionController::get_conversion_result))
            .route("/{job_id}/download/word", web::get().to(ConversionController::download_word_document))
            .route("/{job_id}/download/odt", web::get().to(ConversionController::download_odt_document))
            .route("/{job_id}/download/pdf", web::get().to(ConversionController::download_pdf_document))
    );
}

/*
Route Structure:

1. Protected Routes (Auth Required):
   - POST /api/conversion/upload
   - GET /api/conversion/status/{job_id}
   - GET /api/conversion/history
   - DELETE /api/conversion/{job_id}

Security Flow:
1. Authentication Middleware checks:
   - Valid JWT token in Authorization header
   - Token not expired
   - Token format is correct

Error Responses:
- 401 Unauthorized: Missing/invalid token
- 403 Forbidden: User not found or account not verified
- 404 Not Found: Conversion job not found
- 413 Payload Too Large: File too large
- 415 Unsupported Media Type: Invalid file type
*/ 