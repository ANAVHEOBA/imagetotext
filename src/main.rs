//use actix_web::{App, HttpServer};

mod index;
mod app;
mod config;
mod services;
mod modules;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    println!("Starting Image to Text server...");
    index::start_server().await
}


