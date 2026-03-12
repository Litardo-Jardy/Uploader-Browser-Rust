mod models;
mod routes;
mod middleware;
mod helpers;
mod utils;
mod errors; use axum::{ Router };

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::upload_file::routes())
        .merge(routes::edit_folder::routes())
        .merge(routes::delete_folder::routes())
        .merge(routes::list_folders::routes())
        .merge(routes::create_folder::routes())
        .merge(routes::auth::routes())
        .merge(routes::user::routes());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
