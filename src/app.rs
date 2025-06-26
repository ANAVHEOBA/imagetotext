use actix_web::{web, HttpResponse, Result, middleware::Logger};
use serde::Serialize;
use chrono::{DateTime, Utc};
use std::time::Duration;

use crate::index::START_TIME;
use crate::modules::{configure_routes, configure_conversion_routes};

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    uptime: String,
    timestamp: DateTime<Utc>,
    version: String,
}

pub fn config_app(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/api")
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
            .configure(configure_routes)
            .configure(configure_conversion_routes)
    );
}

async fn health_check() -> Result<HttpResponse> {
    let uptime = START_TIME.elapsed();
    let uptime_str = format_uptime(uptime);
    
    let response = HealthResponse {
        status: "healthy".to_string(),
        uptime: uptime_str,
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

fn format_uptime(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}