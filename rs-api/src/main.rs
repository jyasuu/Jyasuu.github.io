use actix_web::{
    body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::{from_fn, Logger, Next}, web, App, HttpResponse, HttpServer, Responder
};
use actix_cors::Cors;
use env_logger::Env;
use std::env;

mod models;
mod handlers;
mod errors;
mod services;


async fn rate_limit(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    log::info!("rate limit pre-processing...");
    log::info!("peer_addr {:#?}",req.peer_addr());
    log::info!("headers {:#?}",req.headers());
    log::info!("query_string {:#?}",req.query_string());
    log::info!("uri {:#?}",req.uri());
    log::info!("version {:#?}",req.version());
    // pre-processing
    let res = next.call(req).await;
    // post-processing
    log::info!("rate limit post-processing...");
    res
}


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
            .wrap(from_fn(rate_limit))
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