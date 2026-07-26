mod models;
mod routes;
mod middleware;
mod helpers;
mod utils;
mod errors; use axum::{ Router };
use tower_http::services::ServeDir;
use dotenvy::dotenv;
use std::env;
use tower_http::cors::{CorsLayer, Any};
use axum::http::{Method, HeaderValue};
use axum::extract::DefaultBodyLimit;

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);
    dotenv().ok();
    let base_dir = env::var("BASE_DIR").expect("Ruta no definida");

    let app = Router::new()
    .merge(routes::delete_file::routes())
        .merge(routes::list_file::routes())
        .merge(routes::upload_file::routes())
        .merge(routes::edit_folder::routes())
        .merge(routes::delete_folder::routes())
        .merge(routes::list_folders::routes())
        .merge(routes::create_folder::routes())
        .merge(routes::auth::routes())
        .layer(DefaultBodyLimit::max(5 * 1024 * 1024 * 1024))
        .layer(cors)
        .nest_service("", ServeDir::new(base_dir));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
