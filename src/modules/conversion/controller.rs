use actix_multipart::Multipart;
use actix_web::{HttpResponse, Error, HttpRequest, HttpMessage, web};
use futures_util::StreamExt;
use uuid::Uuid;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

use crate::config::database;
use crate::config::environment::Config;
use crate::services::cloudinary::CloudinaryService;
use crate::services::document_converter::DocumentConverter;
use crate::modules::conversion::{
    model::Conversion,
    schema::{UploadResponse, ErrorResponse, MAX_FILE_SIZE, is_allowed_mime_type, ConversionResultResponse},
    crud::ConversionCRUD,
};
use crate::modules::user::crud::UserCRUD;

// --- Structs for Qwen Vision API call ---
#[derive(Serialize)]
struct QwenRequestBody<'a> {
    model: &'a str,
    messages: Vec<QwenMessage<'a>>,
    max_tokens: u32,
}

#[derive(Serialize)]
struct QwenMessage<'a> {
    role: &'a str,
    content: Vec<QwenContentPart<'a>>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum QwenContentPart<'a> {
    #[serde(rename = "text")]
    Text { text: &'a str },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: QwenImageUrl },
}

#[derive(Serialize)]
struct QwenImageUrl {
    url: String,
}

#[derive(Deserialize, Debug)]
struct QwenChatCompletionResponse {
    choices: Vec<QwenChoice>,
}

#[derive(Deserialize, Debug)]
struct QwenChoice {
    message: QwenResponseMessage,
}

#[derive(Deserialize, Debug)]
struct QwenResponseMessage {
    content: String,
}

pub struct ConversionController;

impl ConversionController {
    pub async fn upload_image(mut payload: Multipart, req: HttpRequest) -> Result<HttpResponse, Error> {
        println!("CONTROLLER: Entered upload_image handler.");
        let start_time = Instant::now();

        println!("CONTROLLER: Extracting user claims from token.");
        let claims = match req.extensions().get::<crate::services::jwt::Claims>().cloned() {
            Some(claims) => claims,
            None => {
                println!("CONTROLLER: ERROR - Claims not found in request extensions.");
                return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "Unauthorized".to_string(),
                    message: "User authentication data not found in token.".to_string(),
                }));
            }
        };

        println!("CONTROLLER: Looking up user by UUID: {}", &claims.sub);
        let user = match UserCRUD::find_by_uuid(&claims.sub).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                println!("CONTROLLER: ERROR - User from token not found in DB.");
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    error: "User not found".to_string(),
                    message: "User associated with this token no longer exists.".to_string(),
                }));
            }
            Err(_) => {
                println!("CONTROLLER: ERROR - Database error while fetching user.");
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to retrieve user data.".to_string(),
                }));
            }
        };

        let user_id = match user.id {
            Some(id) => id,
            None => {
                println!("CONTROLLER: ERROR - User record is missing DB ID.");
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal Server Error".to_string(),
                    message: "User record is missing a database ID.".to_string(),
                }));
            }
        };
        println!("CONTROLLER: User authenticated and authorized. User ID: {}", user_id);

        let config = Config::new();
        let cloudinary = CloudinaryService::new(config.clone());
        
        // Process the multipart form data
        println!("CONTROLLER: Processing multipart form data...");
        let mut field = match payload.next().await {
            Some(Ok(field)) => field,
            _ => return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                error: "No file".to_string(),
                message: "No file was uploaded".to_string(),
            })),
        };
        
        let content_disposition = field.content_disposition().ok_or_else(|| actix_web::error::ErrorBadRequest("No content disposition"))?;
        let filename = content_disposition.get_filename().unwrap_or("file").to_string();
        
        // Validate mime type
        let mime_type = field.content_type().ok_or_else(|| actix_web::error::ErrorBadRequest("No mime type"))?.to_string();
        if !is_allowed_mime_type(&mime_type) {
            return Ok(HttpResponse::UnsupportedMediaType().json(ErrorResponse {
                error: "Invalid file type".to_string(),
                message: format!("Supported types: JPEG, PNG, GIF. Got: {}", mime_type),
            }));
        }

        // Write file contents and check size
        let mut file_bytes = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| actix_web::error::ErrorBadRequest(format!("Failed to read chunk: {}", e)))?;
            if (file_bytes.len() + data.len()) as u64 > MAX_FILE_SIZE {
                return Ok(HttpResponse::PayloadTooLarge().json(ErrorResponse {
                    error: "File too large".to_string(),
                    message: format!("Maximum file size is {} bytes", MAX_FILE_SIZE),
                }));
            }
            file_bytes.extend_from_slice(&data);
        }
        let size = file_bytes.len() as u64;
        println!("CONTROLLER: Finished reading file into memory. Size: {} bytes.", size);

        // --- Qwen OCR Step (Replaces Tesseract) ---
        let extracted_text = {
            println!("CONTROLLER: Starting Qwen Vision API call...");
            let base64_image = general_purpose::STANDARD.encode(&file_bytes);
            let image_uri = format!("data:{};base64,{}", mime_type, base64_image);
            
            let api_key = &config.open_router_api_key;
            let model_name = "qwen/qwen-vl-plus";
            let prompt_text = "You are an expert mathematician and a LaTeX specialist. Your task is to accurately transcribe all text and mathematical expressions from the provided image.

**CRITICAL INSTRUCTIONS:**
1.  **Validate and Correct:** Do not just blindly transcribe. Interpret the content. If a mathematical expression seems syntactically incorrect or a symbol is ambiguous, infer the most likely correct version. Your goal is to produce semantically sound mathematics.
2.  **Strict Formatting:** Adhere to the following formatting rules without exception.
    *   **Math Delimiters:** Use ONLY `$...$` for inline math and `$$...$$` for display/block math.
    *   **No Structural LaTeX:** Do NOT use commands like `\\textbf`, `\\textit`, `\\section`, `\\begin{itemize}`, etc. Use plain text for emphasis and structure.
    *   **No Code Blocks:** The entire output must be plain text. Do NOT wrap it in Markdown code blocks (```).
    *   **Plain Lists:** For lists, use simple numbering like `1.`, `2.`, or plain hyphens `-`.

**Example of Correct Output:**
Here is some text with an inline formula $f(x) = x^2 + 1$.

1. A list item with a display formula:
$$ \\int_a^b g(t) dt = G(b) - G(a) $$
2. Another list item.

**Your Goal:** Return ONLY the clean, corrected, and properly formatted transcription from the image.";

            let request_body = QwenRequestBody {
                model: model_name,
                messages: vec![QwenMessage {
                    role: "user",
                    content: vec![
                        QwenContentPart::Text { text: prompt_text },
                        QwenContentPart::ImageUrl { image_url: QwenImageUrl { url: image_uri } },
                    ],
                }],
                max_tokens: 2048,
            };

            let client = reqwest::Client::new();
            let response_result = client
                .post("https://openrouter.ai/api/v1/chat/completions")
                .bearer_auth(api_key)
                .json(&request_body)
                .send()
                .await;

            match response_result {
                Ok(res) if res.status().is_success() => {
                    match res.json::<QwenChatCompletionResponse>().await {
                        Ok(res_body) => {
                            if let Some(choice) = res_body.choices.get(0) {
                                println!("CONTROLLER: Qwen API call successful.");
                                Ok(choice.message.content.clone())
                            } else {
                                Err("Qwen API returned no choices.".to_string())
                            }
                        }
                        Err(e) => Err(format!("Failed to parse Qwen API response: {}", e)),
                    }
                }
                Ok(res) => {
                    let status = res.status();
                    let error_text = res.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
                    Err(format!("Qwen API call failed with status {}: {}", status, error_text))
                }
                Err(e) => Err(format!("Failed to send request to Qwen API: {}", e)),
            }
        };
        
        let extracted_text = match extracted_text {
            Ok(text) => text,
            Err(e) => {
                eprintln!("CONTROLLER: OCR Error: {}", e);
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "OCR failed".to_string(),
                    message: e,
                }));
            }
        };

        // --- Cloudinary Upload Step ---
        println!("CONTROLLER: Starting Cloudinary upload...");
        let temp_dir = std::env::temp_dir().join("imagetotext");
        std::fs::create_dir_all(&temp_dir).ok();
        let temp_path = temp_dir.join(format!("{}-{}", Uuid::new_v4(), &filename));
        if std::fs::write(&temp_path, &file_bytes).is_err() {
             return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal Error".to_string(),
                message: "Failed to write temporary file for upload.".to_string(),
            }));
        }

        let public_id = cloudinary.generate_public_id(&filename);
        let cloudinary_url = match cloudinary.upload_image(&temp_path, &public_id).await {
            Ok(url) => {
                std::fs::remove_file(&temp_path).ok(); // Clean up temp file
                url
            },
            Err(e) => {
                std::fs::remove_file(&temp_path).ok();
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Upload failed".to_string(),
                    message: e,
                }));
            }
        };
        println!("CONTROLLER: Cloudinary upload finished.");

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        // --- Database Record Creation ---
        println!("CONTROLLER: Creating database record...");
        let mut conversion = Conversion::new(user_id, filename, size, mime_type);
        conversion.mark_completed(extracted_text, processing_time_ms, cloudinary_url, public_id.clone());

        let collection = match database::get_conversion_collection().await {
            Ok(coll) => coll,
            Err(e) => {
                let _ = cloudinary.delete_image(&public_id).await; // Cleanup cloudinary on DB error
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    message: format!("Failed to get collection: {}", e),
                }));
            }
        };

        let job_id = conversion.job_id.clone();
        match ConversionCRUD::create(&conversion, &collection).await {
            Ok(_) => {
                if let Err(e) = UserCRUD::increment_conversion_count(&user.uuid).await {
                    println!("CONTROLLER: WARNING - Failed to increment user conversion count: {:?}", e);
                }
                println!("CONTROLLER: Successfully created record. Returning response.");
                Ok(HttpResponse::Ok().json(UploadResponse {
                    job_id,
                    message: "Image processed and uploaded successfully.".to_string(),
                    status: "completed".to_string(),
                }))
            }
            Err(e) => {
                let _ = cloudinary.delete_image(&public_id).await;
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    message: format!("Failed to create record: {}", e),
                }))
            }
        }
    }

    pub async fn get_conversion_result(req: HttpRequest, job_id: web::Path<String>) -> Result<HttpResponse, Error> {
        println!("CONTROLLER: Entered get_conversion_result handler for job_id: {}", job_id);

        let claims = match req.extensions().get::<crate::services::jwt::Claims>().cloned() {
            Some(claims) => claims,
            None => {
                return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "Unauthorized".to_string(),
                    message: "User authentication data not found.".to_string(),
                }));
            }
        };

        let user = match UserCRUD::find_by_uuid(&claims.sub).await {
            Ok(Some(user)) => user,
            _ => {
                return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "Unauthorized".to_string(),
                    message: "Invalid user token.".to_string(),
                }));
            }
        };
        let user_db_id = user.id.unwrap();

        let collection = match database::get_conversion_collection().await {
            Ok(coll) => coll,
            Err(_) => return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database error".to_string(),
                message: "Failed to connect to database.".to_string(),
            }))
        };
        
        match ConversionCRUD::find_by_job_id(&job_id, &collection).await {
            Ok(Some(conversion)) => {
                if conversion.user_id != user_db_id {
                    return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                        error: "Forbidden".to_string(),
                        message: "You do not have permission to access this resource.".to_string(),
                    }));
                }
                
                let response = ConversionResultResponse {
                    job_id: conversion.job_id,
                    status: conversion.status.to_string(),
                    original_filename: conversion.original_filename,
                    extracted_text: conversion.extracted_text.unwrap_or_default(),
                    created_at: conversion.created_at.to_string(),
                    processing_time_ms: conversion.processing_time_ms.unwrap_or(0),
                };

                Ok(HttpResponse::Ok().json(response))
            }
            Ok(None) => Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: "Not Found".to_string(),
                message: "Conversion job with this ID was not found.".to_string(),
            })),
            Err(_) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database error".to_string(),
                message: "Failed to retrieve conversion data.".to_string(),
            })),
        }
    }

    pub async fn download_word_document(req: HttpRequest, job_id: web::Path<String>) -> Result<HttpResponse, Error> {
        println!("CONTROLLER: Entered download_word_document handler for job_id: {}", job_id);

        let claims = match req.extensions().get::<crate::services::jwt::Claims>().cloned() {
            Some(claims) => claims,
            None => return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Unauthorized".to_string(),
                message: "User authentication data not found.".to_string(),
            })),
        };

        let user = match UserCRUD::find_by_uuid(&claims.sub).await {
            Ok(Some(user)) => user,
            _ => return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Unauthorized".to_string(),
                message: "Invalid user token.".to_string(),
            })),
        };

        let collection = match database::get_conversion_collection().await {
            Ok(coll) => coll,
            Err(_) => return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database Error".to_string(),
                message: "Failed to connect to database.".to_string(),
            })),
        };

        match ConversionCRUD::find_by_job_id(&job_id, &collection).await {
            Ok(Some(conversion)) if conversion.user_id == user.id.unwrap() => {
                let latex_content = conversion.extracted_text.unwrap_or_default();
                if latex_content.is_empty() {
                    return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        error: "No Content".to_string(),
                        message: "The conversion has no text to export.".to_string(),
                    }));
                }

                match DocumentConverter::latex_to_docx(&latex_content) {
                    Ok(docx_bytes) => {
                        let filename = format!("{}.docx", conversion.original_filename);
                        Ok(HttpResponse::Ok()
                            .content_type("application/vnd.openxmlformats-officedocument.wordprocessingml.document")
                            .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
                            .body(docx_bytes))
                    }
                    Err(e) => {
                        eprintln!("Word conversion failed: {}", e);
                        Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "Document Conversion Failed".to_string(),
                            message: format!("Could not convert LaTeX to DOCX. Error: {}", e),
                        }))
                    }
                }
            }
            Ok(_) => Ok(HttpResponse::Forbidden().json(ErrorResponse {
                error: "Forbidden".to_string(),
                message: "You do not have permission to access this resource.".to_string(),
            })),
            Err(_) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database Error".to_string(),
                message: "Failed to retrieve conversion data.".to_string(),
            })),
        }
    }

    pub async fn download_odt_document(req: HttpRequest, job_id: web::Path<String>) -> Result<HttpResponse, Error> {
        println!("CONTROLLER: Entered download_odt_document handler for job_id: {}", job_id);

        let claims = match req.extensions().get::<crate::services::jwt::Claims>().cloned() {
            Some(claims) => claims,
            None => return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Unauthorized".to_string(),
                message: "User authentication data not found.".to_string(),
            })),
        };

        let user = match UserCRUD::find_by_uuid(&claims.sub).await {
            Ok(Some(user)) => user,
            _ => return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Unauthorized".to_string(),
                message: "Invalid user token.".to_string(),
            })),
        };

        let collection = match database::get_conversion_collection().await {
            Ok(coll) => coll,
            Err(_) => return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database Error".to_string(),
                message: "Failed to connect to database.".to_string(),
            })),
        };

        match ConversionCRUD::find_by_job_id(&job_id, &collection).await {
            Ok(Some(conversion)) if conversion.user_id == user.id.unwrap() => {
                let latex_content = conversion.extracted_text.unwrap_or_default();
                if latex_content.is_empty() {
                    return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        error: "No Content".to_string(),
                        message: "The conversion has no text to export.".to_string(),
                    }));
                }

                match DocumentConverter::latex_to_odt(&latex_content) {
                    Ok(odt_bytes) => {
                        let filename = format!("{}.odt", conversion.original_filename);
                        Ok(HttpResponse::Ok()
                            .content_type("application/vnd.oasis.opendocument.text")
                            .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
                            .body(odt_bytes))
                    }
                    Err(e) => {
                        eprintln!("ODT conversion failed: {}", e);
                        Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "Document Conversion Failed".to_string(),
                            message: format!("Could not convert LaTeX to ODT. Error: {}", e),
                        }))
                    }
                }
            }
            Ok(_) => Ok(HttpResponse::Forbidden().json(ErrorResponse {
                error: "Forbidden".to_string(),
                message: "You do not have permission to access this resource.".to_string(),
            })),
            Err(_) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database Error".to_string(),
                message: "Failed to retrieve conversion data.".to_string(),
            })),
        }
    }

    pub async fn download_pdf_document(req: HttpRequest, job_id: web::Path<String>) -> Result<HttpResponse, Error> {
        println!("CONTROLLER: Entered download_pdf_document handler for job_id: {}", job_id);

        let claims = match req.extensions().get::<crate::services::jwt::Claims>().cloned() {
            Some(claims) => claims,
            None => return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Unauthorized".to_string(),
                message: "User authentication data not found.".to_string(),
            })),
        };

        let user = match UserCRUD::find_by_uuid(&claims.sub).await {
            Ok(Some(user)) => user,
            _ => return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Unauthorized".to_string(),
                message: "Invalid user token.".to_string(),
            })),
        };

        let collection = match database::get_conversion_collection().await {
            Ok(coll) => coll,
            Err(_) => return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database Error".to_string(),
                message: "Failed to connect to database.".to_string(),
            })),
        };

        match ConversionCRUD::find_by_job_id(&job_id, &collection).await {
            Ok(Some(conversion)) if conversion.user_id == user.id.unwrap() => {
                let latex_content = conversion.extracted_text.unwrap_or_default();
                if latex_content.is_empty() {
                    return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        error: "No Content".to_string(),
                        message: "The conversion has no text to export.".to_string(),
                    }));
                }

                match DocumentConverter::latex_to_pdf(&latex_content) {
                    Ok(pdf_bytes) => {
                        let filename = format!("{}.pdf", conversion.original_filename);
                        Ok(HttpResponse::Ok()
                            .content_type("application/pdf")
                            .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
                            .body(pdf_bytes))
                    }
                    Err(e) => {
                        eprintln!("PDF conversion failed: {}", e);
                        Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "Document Conversion Failed".to_string(),
                            message: format!("Could not convert LaTeX to PDF. Error: {}", e),
                        }))
                    }
                }
            }
            Ok(_) => Ok(HttpResponse::Forbidden().json(ErrorResponse {
                error: "Forbidden".to_string(),
                message: "You do not have permission to access this resource.".to_string(),
            })),
            Err(_) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database Error".to_string(),
                message: "Failed to retrieve conversion data.".to_string(),
            })),
        }
    }
} 