use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};
use crate::helpers::delete_file::delete_file;
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
       .route("/delete_file", post(handle_delete_file))
}

async fn handle_delete_file(
    Json(body): Json<FolderInput>
) -> Result<Json<FolderResponse>,ApiError> {
 
   delete_file(&body.name).await?;
   Ok(Json(FolderResponse {
        message_status: "Archivo eliminada con éxito".to_string()
    }))
}
