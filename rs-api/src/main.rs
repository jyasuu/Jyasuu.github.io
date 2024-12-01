use actix_web::{
    web, 
    App, 
    HttpServer, 
    middleware::Logger,
    HttpResponse, 
    Responder
};
use actix_cors::Cors;
use env_logger::Env;
use std::env;

mod models;
mod handlers;
mod errors;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load environment variables
    dotenv::dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_addr = format!("{}:{}", host, port);

    println!("🚀 Server starting on {}", server_addr);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .route("/chat/ollama", web::post().to(handlers::ollama_chat_handler))
                    .route("/chat/xai", web::post().to(handlers::xai_chat_handler))
                    .route("/models", web::get().to(handlers::list_models))
            )
    })
    .bind(server_addr)?
    .run()
    .await
}