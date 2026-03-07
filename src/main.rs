mod models;
mod routes;
mod middleware;
mod helpers;
use axum::{ Router };

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::create_folder::routes())
        .merge(routes::auth::routes())
        .merge(routes::user::routes());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
