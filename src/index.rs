use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use std::time::Instant;
use std::sync::Arc;

use crate::app;
use crate::config::environment::Config;

pub static START_TIME: once_cell::sync::Lazy<Arc<Instant>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Instant::now()));

pub async fn start_server() -> std::io::Result<()> {
    let config = Config::new();
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("127.0.0.1:{}", port);
    
    println!("Server starting on http://{}", address);
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.frontend_url)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization", "Accept"])
            .expose_headers(vec!["x-refresh-token"])  
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .configure(app::config_app)
    })
    .bind(address)?
    .run()
    .await
}