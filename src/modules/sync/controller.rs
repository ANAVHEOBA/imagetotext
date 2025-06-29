use actix_web::{web, HttpResponse, Error, HttpRequest, HttpMessage};
use mongodb::bson::doc;
use crate::config::database;
use crate::modules::sync::{
    crud::SyncCRUD,
    schema::{ProjectListResponse, ProjectListResult, ErrorResponse, CreateProjectRequest, AssignConversionsRequest, ProjectConversionsResponse, ConversionListItem, SuccessResponse},
    model::Project,
};
use crate::modules::user::crud::UserCRUD;
use crate::modules::conversion::model::Conversion;
use serde::Deserialize;
use mongodb::{Collection, bson::oid::ObjectId};

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<i64>,
    limit: Option<i64>,
}

pub struct SyncController;

impl SyncController {
    pub async fn create_project(
        req: HttpRequest,
        project_data: web::Json<CreateProjectRequest>
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

        // Get projects collection
        let collection: Collection<Project> = match database::get_projects_collection().await {
            Ok(collection) => collection,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to connect to database.".to_string(),
                }));
            }
        };

        // Create project
        match SyncCRUD::create_project(
            &user_id,
            project_data.name.clone(),
            project_data.description.clone(),
            &collection
        ).await {
            Ok(project) => {
                let response = ProjectListResponse {
                    id: project.id.unwrap().to_hex(),
                    name: project.name,
                    description: project.description,
                    cloudinary_folder: project.cloudinary_folder,
                    conversion_count: project.conversion_count,
                    total_storage_bytes: project.total_storage_bytes,
                    created_at: project.created_at.to_string(),
                    updated_at: project.updated_at.to_string(),
                };
                Ok(HttpResponse::Created().json(response))
            }
            Err(_) => {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to create project.".to_string(),
                }))
            }
        }
    }
    
    pub async fn list_projects(req: HttpRequest) -> Result<HttpResponse, Error> {
        // Default to first page with 10 items
        let params = PaginationParams {
            page: Some(1),
            limit: Some(10),
        };
        Self::list_projects_with_pagination(req, params).await
    }

    // New paginated endpoint
    pub async fn list_projects_paginated(
        req: HttpRequest,
        path: web::Path<(i64, i64)>,
    ) -> Result<HttpResponse, Error> {
        let (page, limit) = path.into_inner();
        let params = PaginationParams {
            page: Some(page),
            limit: Some(limit),
        };
        Self::list_projects_with_pagination(req, params).await
    }

    // Internal method to handle pagination
    async fn list_projects_with_pagination(
        req: HttpRequest,
        params: PaginationParams,
    ) -> Result<HttpResponse, Error> {
        println!("CONTROLLER: Entered list_projects handler with pagination.");

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

        // Get projects collection
        let collection: Collection<Project> = match database::get_projects_collection().await {
            Ok(collection) => collection,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to connect to database.".to_string(),
                }));
            }
        };

        // Calculate pagination values
        let page = params.page.unwrap_or(1).max(1);
        let limit = params.limit.unwrap_or(10).clamp(1, 100);
        let skip = (page - 1) * limit;

        // Get projects and count with pagination
        let projects = match SyncCRUD::list_projects_paginated(&user_id, skip, limit, &collection).await {
            Ok(projects) => projects,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to retrieve projects.".to_string(),
                }));
            }
        };

        let total = match SyncCRUD::count_user_projects(&user_id, &collection).await {
            Ok(count) => count,
            Err(_) => 0, // Default to 0 if count fails
        };

        // Convert projects to response format
        let project_responses: Vec<ProjectListResponse> = projects
            .into_iter()
            .map(|project| ProjectListResponse {
                id: project.id.unwrap().to_hex(),
                name: project.name,
                description: project.description,
                cloudinary_folder: project.cloudinary_folder,
                conversion_count: project.conversion_count,
                total_storage_bytes: project.total_storage_bytes,
                created_at: project.created_at.to_string(),
                updated_at: project.updated_at.to_string(),
            })
            .collect();

        Ok(HttpResponse::Ok().json(ProjectListResult {
            projects: project_responses,
            total,
            page,
            limit,
            total_pages: (total as f64 / limit as f64).ceil() as i64,
        }))
    }

    pub async fn list_project_conversions(
        req: HttpRequest,
        path: web::Path<(String, i64, i64)>,
    ) -> Result<HttpResponse, Error> {
        let (project_id, page, limit) = path.into_inner();
        let project_id = match ObjectId::parse_str(&project_id) {
            Ok(id) => id,
            Err(_) => {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid Project ID".to_string(),
                    message: "The provided project ID is not valid.".to_string(),
                }));
            }
        };

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

        // Verify project exists and belongs to user
        let projects_collection: Collection<Project> = match database::get_projects_collection().await {
            Ok(collection) => collection,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to connect to database.".to_string(),
                }));
            }
        };

        if let Ok(None) = SyncCRUD::find_project(&project_id, &user_id, &projects_collection).await {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: "Project not found".to_string(),
                message: "The specified project does not exist or does not belong to you.".to_string(),
            }));
        }

        // Get conversions collection
        let conversions_collection: Collection<Conversion> = match database::get_conversion_collection().await {
            Ok(collection) => collection,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to connect to database.".to_string(),
                }));
            }
        };

        let skip = (page - 1) * limit;
        
        // Get conversions
        let conversions = match SyncCRUD::list_project_conversions(
            &project_id,
            &user_id,
            skip,
            limit,
            &conversions_collection
        ).await {
            Ok(conversions) => conversions,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to retrieve conversions.".to_string(),
                }));
            }
        };

        // Convert to response format
        let conversion_responses: Vec<ConversionListItem> = conversions
            .into_iter()
            .map(|conv| ConversionListItem {
                job_id: conv.job_id,
                original_filename: conv.original_filename,
                cloudinary_url: conv.cloudinary_url,
                file_size: conv.file_size,
                status: conv.status.to_string(),
                created_at: conv.created_at.to_string(),
                completed_at: conv.completed_at.map(|dt| dt.to_string()),
            })
            .collect();

        let total = match conversions_collection.count_documents(
            doc! {
                "user_id": user_id,
                "project_id": project_id
            }
        ).await {
            Ok(count) => count as i64,
            Err(_) => 0,
        };

        Ok(HttpResponse::Ok().json(ProjectConversionsResponse {
            conversions: conversion_responses,
            total,
            page,
            limit,
            total_pages: (total as f64 / limit as f64).ceil() as i64,
        }))
    }

    pub async fn list_unassigned_conversions(
        req: HttpRequest,
        path: web::Path<(i64, i64)>,
    ) -> Result<HttpResponse, Error> {
        let (page, limit) = path.into_inner();

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

        // Get conversions collection
        let conversions_collection: Collection<Conversion> = match database::get_conversion_collection().await {
            Ok(collection) => collection,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to connect to database.".to_string(),
                }));
            }
        };

        let skip = (page - 1) * limit;
        
        // Get unassigned conversions
        let conversions = match SyncCRUD::list_unassigned_conversions(
            &user_id,
            skip,
            limit,
            &conversions_collection
        ).await {
            Ok(conversions) => conversions,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to retrieve conversions.".to_string(),
                }));
            }
        };

        // Convert to response format
        let conversion_responses: Vec<ConversionListItem> = conversions
            .into_iter()
            .map(|conv| ConversionListItem {
                job_id: conv.job_id,
                original_filename: conv.original_filename,
                cloudinary_url: conv.cloudinary_url,
                file_size: conv.file_size,
                status: conv.status.to_string(),
                created_at: conv.created_at.to_string(),
                completed_at: conv.completed_at.map(|dt| dt.to_string()),
            })
            .collect();

        let total = match conversions_collection.count_documents(
            doc! {
                "user_id": user_id,
                "project_id": null
            }
        ).await {
            Ok(count) => count as i64,
            Err(_) => 0,
        };

        Ok(HttpResponse::Ok().json(ProjectConversionsResponse {
            conversions: conversion_responses,
            total,
            page,
            limit,
            total_pages: (total as f64 / limit as f64).ceil() as i64,
        }))
    }

    pub async fn assign_conversions(
        req: HttpRequest,
        path: web::Path<String>,
        data: web::Json<AssignConversionsRequest>,
    ) -> Result<HttpResponse, Error> {
        let project_id = match ObjectId::parse_str(&path.into_inner()) {
            Ok(id) => id,
            Err(_) => {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid Project ID".to_string(),
                    message: "The provided project ID is not valid.".to_string(),
                }));
            }
        };

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
        let projects_collection: Collection<Project> = match database::get_projects_collection().await {
            Ok(collection) => collection,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to connect to database.".to_string(),
                }));
            }
        };

        let conversions_collection: Collection<Conversion> = match database::get_conversion_collection().await {
            Ok(collection) => collection,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: "Failed to connect to database.".to_string(),
                }));
            }
        };

        // Verify project exists and belongs to user
        if let Ok(None) = SyncCRUD::find_project(&project_id, &user_id, &projects_collection).await {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: "Project not found".to_string(),
                message: "The specified project does not exist or does not belong to you.".to_string(),
            }));
        }

        // Assign conversions
        match SyncCRUD::assign_conversions_to_project(
            &project_id,
            &user_id,
            &data.conversion_ids,
            &conversions_collection,
            &projects_collection
        ).await {
            Ok(_) => Ok(HttpResponse::Ok().json(SuccessResponse {
                message: "Conversions assigned successfully".to_string(),
                data: None,
            })),
            Err(_) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database Error".to_string(),
                message: "Failed to assign conversions to project.".to_string(),
            })),
        }
    }
}