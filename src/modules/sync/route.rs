use actix_web::web;
use crate::modules::sync::controller::SyncController;
use crate::middleware::user_auth::Authentication;

pub fn configure_sync_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sync")
            .wrap(Authentication::new())  // Protect all sync routes
            .route("/projects", web::get().to(SyncController::list_projects))
            .route("/projects", web::post().to(SyncController::create_project))
            .route("/projects/{page}/{limit}", web::get().to(SyncController::list_projects_paginated))
            .route("/projects/{project_id}/conversions/{page}/{limit}", web::get().to(SyncController::list_project_conversions))
            .route("/projects/{project_id}/assign", web::post().to(SyncController::assign_conversions))
            .route("/conversions/unassigned/{page}/{limit}", web::get().to(SyncController::list_unassigned_conversions))
    );
}
