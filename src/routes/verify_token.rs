use axum::{Router, routing::get, Json};
use serde::Serialize;
use crate::middleware::auth::UsuarioAutenticado;

#[derive(Serialize)]
struct VerifyResponse {
    usuario: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/me", get(handle_verify))
}

async fn handle_verify(
    user: UsuarioAutenticado,  
) -> Json<VerifyResponse> {
    Json(VerifyResponse { usuario: user.usuario})}
