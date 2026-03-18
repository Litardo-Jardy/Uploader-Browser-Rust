mod models;
mod routes;
mod middleware;
mod helpers;
mod utils;
mod errors; use axum::{ Router };
use tower_http::services::ServeDir;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let base_dir = env::var("BASE_DIR").expect("Ruta no definida");

    let app = Router::new()
    .merge(routes::delete_file::routes())
        .merge(routes::upload_file::routes())
        .merge(routes::edit_folder::routes())
        .merge(routes::delete_folder::routes())
        .merge(routes::list_folders::routes())
        .merge(routes::create_folder::routes())
        .merge(routes::auth::routes())
        .nest_service("/files", ServeDir::new(base_dir));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
