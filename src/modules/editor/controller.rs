use actix_web::{web, HttpResponse, Error, HttpRequest, HttpMessage};
use mongodb::Collection;
use crate::config::database;
use crate::modules::user::crud::UserCRUD;
use crate::modules::conversion::crud::ConversionCRUD;
use crate::services::document_converter::DocumentConverter;
use crate::modules::editor::{
    crud::EditorCRUD,
    model::Preview,
    schema::{PreviewResponse, DocumentMetadata, ErrorResponse, PreviewListResponse, PreviewSummary},
};

#[derive(serde::Deserialize)]
pub struct PaginationParams {
    page: Option<i64>,
    limit: Option<i64>,
}

pub struct EditorController;

impl EditorController {
    pub async fn get_preview(
        req: HttpRequest,
        path: web::Path<String>,
    ) -> Result<HttpResponse, Error> {
        let conversion_id = path.into_inner();

        // Get user from token
        let claims = match req.extensions().get::<crate::services::jwt::Claims>().cloned() {
            Some(claims) => claims,
            None => {
                return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "Unauthorized".to_string(),
                    message: "User authentication data not found.".to_string(),
                }));
            }
        };

        // Get user from database
        let user = match UserCRUD::find_by_uuid(&claims.sub).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    error: "User not found".to_string(),
                    message: "User associated with this token no longer exists.".to_string(),
                }));
            }
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to retrieve user data.".to_string(),
                }));
            }
        };

        let user_id = match user.id {
            Some(id) => id,
            None => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Invalid User".to_string(),
                    message: "User record is missing required data.".to_string(),
                }));
            }
        };

        // Get collections
        let previews_collection: Collection<Preview> = match database::get_previews_collection().await {
            Ok(collection) => collection,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to connect to database.".to_string(),
                }));
            }
        };

        // Check if preview exists
        match EditorCRUD::get_preview(&user_id, &conversion_id, &previews_collection).await {
            Ok(Some(preview)) => {
                let response = PreviewResponse {
                    html_content: preview.html_content,
                    metadata: DocumentMetadata {
                        total_pages: preview.total_pages,
                        word_count: preview.word_count,
                        last_modified: preview.updated_at,
                        format: preview.format.to_string(),
                        file_size: None,  // We don't have the size for existing previews
                        download_filename: Some(format!("{}.html", preview.original_filename)),
                    },
                    conversion_id: preview.conversion_id,
                    status: preview.preview_status.to_string(),
                };
                Ok(HttpResponse::Ok().json(response))
            }
            Ok(None) => {
                // If preview doesn't exist, try to generate it from conversion
                let conversions_collection = match database::get_conversion_collection().await {
                    Ok(collection) => collection,
                    Err(_) => {
                        return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "Database Error".to_string(),
                            message: "Failed to connect to database.".to_string(),
                        }));
                    }
                };

                match ConversionCRUD::find_by_job_id(&conversion_id, &conversions_collection).await {
                    Ok(Some(conversion)) => {
                        let text_content = conversion.extracted_text.unwrap_or_default();
                        // Generate HTML preview
                        match DocumentConverter::latex_to_html_preview(&text_content) {
                            Ok(result) => {
                                // Create preview
                                match EditorCRUD::create_preview(
                                    &user_id,
                                    &conversion_id,
                                    &String::from_utf8_lossy(&result.content),
                                    &conversion.original_filename,
                                    &previews_collection,
                                ).await {
                                    Ok(preview) => {
                                        let response = PreviewResponse {
                                            html_content: preview.html_content,
                                            metadata: DocumentMetadata {
                                                total_pages: preview.total_pages,
                                                word_count: preview.word_count,
                                                last_modified: preview.updated_at,
                                                format: preview.format.to_string(),
                                                file_size: Some(result.size),
                                                download_filename: Some(format!("{}.html", preview.original_filename)),
                                            },
                                            conversion_id: preview.conversion_id,
                                            status: preview.preview_status.to_string(),
                                        };
                                        Ok(HttpResponse::Ok().json(response))
                                    }
                                    Err(_) => {
                                        Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                                            error: "Preview Creation Failed".to_string(),
                                            message: "Failed to save preview.".to_string(),
                                        }))
                                    }
                                }
                            }
                            Err(_) => {
                                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                                    error: "Preview Generation Failed".to_string(),
                                    message: "Failed to generate HTML preview.".to_string(),
                                }))
                            }
                        }
                    }
                    Ok(None) => {
                        Ok(HttpResponse::NotFound().json(ErrorResponse {
                            error: "Conversion not found".to_string(),
                            message: "The specified conversion does not exist.".to_string(),
                        }))
                    }
                    Err(_) => {
                        Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "Database Error".to_string(),
                            message: "Failed to retrieve conversion data.".to_string(),
                        }))
                    }
                }
            }
            Err(_) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to retrieve preview data.".to_string(),
                }))
            }
        }
    }

    pub async fn list_previews(
        req: HttpRequest,
        query: web::Query<PaginationParams>,
    ) -> Result<HttpResponse, Error> {
        // Get user from token
        let claims = match req.extensions().get::<crate::services::jwt::Claims>().cloned() {
            Some(claims) => claims,
            None => {
                return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "Unauthorized".to_string(),
                    message: "User authentication data not found.".to_string(),
                }));
            }
        };

        // Get user from database
        let user = match UserCRUD::find_by_uuid(&claims.sub).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    error: "User not found".to_string(),
                    message: "User associated with this token no longer exists.".to_string(),
                }));
            }
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to retrieve user data.".to_string(),
                }));
            }
        };

        let user_id = match user.id {
            Some(id) => id,
            None => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Invalid User".to_string(),
                    message: "User record is missing required data.".to_string(),
                }));
            }
        };

        // Get previews collection
        let previews_collection: Collection<Preview> = match database::get_previews_collection().await {
            Ok(collection) => collection,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to connect to database.".to_string(),
                }));
            }
        };

        // Calculate pagination values
        let page = query.page.unwrap_or(1).max(1);
        let limit = query.limit.unwrap_or(10).clamp(1, 100);
        let skip = (page - 1) * limit;

        // Get previews with pagination
        let previews = match EditorCRUD::list_previews(&user_id, skip, limit, &previews_collection).await {
            Ok(previews) => previews,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to retrieve previews.".to_string(),
                }));
            }
        };

        let total = match EditorCRUD::count_user_previews(&user_id, &previews_collection).await {
            Ok(count) => count,
            Err(_) => 0,
        };

        // Convert to response format
        let preview_summaries: Vec<PreviewSummary> = previews
            .into_iter()
            .map(|preview| PreviewSummary {
                conversion_id: preview.conversion_id,
                original_filename: preview.original_filename,
                preview_status: preview.preview_status.to_string(),
                created_at: preview.created_at,
                last_modified: preview.updated_at,
                file_size: None,  // We don't store file size for existing previews
            })
            .collect();

        Ok(HttpResponse::Ok().json(PreviewListResponse {
            previews: preview_summaries,
            total,
            page,
            limit,
            total_pages: (total as f64 / limit as f64).ceil() as i64,
        }))
    }
}