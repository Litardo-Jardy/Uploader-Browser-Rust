use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};
use crate::helpers::edit_folder::edit_folder;
use crate::errors::api_error::ApiError;

#[derive(Deserialize)]
struct FolderInput {
  path: String,
  new_path: String,
}

#[derive(Serialize)]
struct FolderResponse {
  message_status: String,
}

pub fn routes() -> Router {
   Router::new()
    .route("/edit_element", post(handle_edit_folder))
}

async fn handle_edit_folder( Json(body): Json<FolderInput> ) -> Result<Json<FolderResponse>, ApiError> {

   edit_folder(&body.path, &body.new_path).await?;

   let message = format!("Carpeta '{}' se edito con existo a '{}'.",&body.path, &body.new_path).to_string();
   Ok(Json(FolderResponse { 
         message_status: message }))
}
