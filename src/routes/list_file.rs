use axum::{Json, Router, routing::get, extract::Query};
use serde::{Deserialize, Serialize};
use crate::helpers::list_file::list_file;
use crate::helpers::list_file::FileInfo;
use crate::errors::api_error::ApiError;

#[derive(Deserialize)]
struct FileInput {
    path: String,
}

#[derive(Serialize)]
struct FilesResponse {
    files: Vec<FileInfo>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/list_files", get(handle_list_folders))
}

async fn handle_list_folders(
    Query(params): Query<FileInput>
) -> Result<Json<FilesResponse>, ApiError> {
    let files = list_file(&params.path).await?;
    Ok(Json(FilesResponse { files }))
}
