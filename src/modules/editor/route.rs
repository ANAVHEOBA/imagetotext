use actix_web::web;
use crate::modules::editor::controller::EditorController;
use crate::middleware::user_auth::Authentication;

pub fn configure_editor_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/editor")
            .wrap(Authentication::new())  // Protect all editor routes
            .route("/preview/{conversion_id}", web::get().to(EditorController::get_preview))
            .route("/previews", web::get().to(EditorController::list_previews))
    );
}
