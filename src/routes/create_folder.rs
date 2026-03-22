use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};
use crate::helpers::create_folder::add_folder;
use crate::errors::api_error::ApiError;
use crate::middleware::auth::UsuarioAutenticado;

#[derive(Deserialize)]
struct FolderInput {
  path: String,
}

#[derive(Serialize)]
struct FolderResponse {
  message_status: String,
}

pub fn routes() -> Router {
   Router::new()
       .route("/create_folder", post(handle_create_folder))
}

async fn handle_create_folder(
    _user: UsuarioAutenticado, 
    Json(body): Json<FolderInput> ) -> Result<Json<FolderResponse>, ApiError> {

  add_folder(&body.path).await?;
  Ok(Json(FolderResponse { message_status: ("Carpeta creada con existo.".to_string()) }))

} 

