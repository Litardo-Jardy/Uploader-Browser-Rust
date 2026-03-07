use axum::{Router, routing::post, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::helpers::create_folder::add_folder;
//Import de la validacion del token
//use crate::middleware::auth::UsuarioAutenticado;

#[derive(Deserialize)]
struct FolderInput {
  name: String,
}

#[derive(Serialize)]
struct FolderResponse {
  message_status: String,
}

pub fn routes() -> Router {
   Router::new()
       .route("/create_folder", post(handle_create_folder))
}

// NO olvidar que aqui debe ir la validacion del token "_user: UsuarioAutenticado" se a quitado por
// temas de practicas;
async fn handle_create_folder( Json(body): Json<FolderInput> ) -> Result<Json<FolderResponse>, StatusCode> {

  add_folder(&body.name).await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
  println!("Carpeta creada exitosamente");
  Ok(Json(FolderResponse { message_status: ("Carpeta creada con existo.".to_string()) }))
} 

