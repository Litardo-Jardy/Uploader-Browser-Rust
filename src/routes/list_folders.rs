use axum::{Json, Router, routing::{post}};
use serde::{Deserialize, Serialize};
use crate::helpers::list_folders::list_folders;
use crate::helpers::list_folders::FolderInfo;
use crate::errors::api_error::ApiError;

#[derive(Deserialize)]
struct FolderInput{
   path: String,
}

#[derive(Serialize)]
struct FolderResponse{
   folders: Vec<FolderInfo>, 
}

pub fn routes() -> Router {
    Router::new()
        .route("/list_folders", post(handle_list_folders))
}

async fn handle_list_folders( Json(body): Json<FolderInput> ) -> Result<Json<FolderResponse>, ApiError> { 

    let folders = list_folders(&body.path).await?;
    Ok(Json(FolderResponse { folders }))

}
