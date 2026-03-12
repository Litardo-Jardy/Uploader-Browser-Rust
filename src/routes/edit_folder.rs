use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};
use crate::helpers::edit_folder::edit_folder;
use crate::errors::api_error::ApiError;

#[derive(Deserialize)]
struct FolderInput {
  name: String,
  new_name: String,
}

#[derive(Serialize)]
struct FolderResponse {
  message_status: String,
}

pub fn routes() -> Router {
   Router::new()
    .route("/edit_folder", post(handle_edit_folder))
}

async fn handle_edit_folder( Json(body): Json<FolderInput> ) -> Result<Json<FolderResponse>, ApiError> {

   edit_folder(&body.name, &body.new_name).await?;

   let message = format!("Carpeta '{}' se edito con existo a '{}'.",&body.name, &body.new_name).to_string();
   Ok(Json(FolderResponse { 
         message_status: message }))
}
