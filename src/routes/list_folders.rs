use axum::{Json, Router, routing::{post}};
use serde::{Deserialize, Serialize};
use crate::helpers::list_folders::list_folders;
use crate::errors::api_error::ApiError;

#[derive(Deserialize)]
struct FolderInput{
   name: String,
}

#[derive(Serialize)]
struct FolderNameResponse{
   folders: Vec<String>, 
}

pub fn routes() -> Router {
    Router::new()
        .route("/list_folders", post(handle_list_folders))
}

async fn handle_list_folders( Json(body): Json<FolderInput> ) -> Result<Json<FolderNameResponse>, ApiError> { 

    let folders = list_folders(&body.name).await?;
    Ok(Json(FolderNameResponse { folders }))

}
