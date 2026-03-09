use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};
use crate::helpers::delete_folder::delete_folder;
use crate::errors::api_error::ApiError;

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
       .route("/delete_folder", post(handle_delete_folder))
}

async fn handle_delete_folder(
    Json(body): Json<FolderInput>
) -> Result<Json<FolderResponse>,ApiError> {

   delete_folder(&body.name).await?;
   Ok(Json(FolderResponse {
        message_status: "Carpeta eliminada con éxito".to_string()
    }))
}
