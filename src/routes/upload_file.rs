use axum::{Router, routing::post, Json };
use serde::{ Serialize };
use crate::helpers::upload_file::upload_file;
use crate::errors::api_error::ApiError;
use axum::extract::Multipart;
use crate::middleware::auth::UsuarioAutenticado;

#[derive(Serialize)]
struct DataResponde {
   message_status: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/upload_file", post(handle_uploader_file))
}

async fn handle_uploader_file(
    _user: UsuarioAutenticado,
    mut multipart: Multipart
) -> Result<Json<DataResponde>, ApiError> {

    let mut name = String::new();
    let mut route = String::new();
    let mut content: Vec<u8> = Vec::new();

    while let Some(field) = multipart.next_field().await?
    {
        match field.name() {
            Some("name") => name = field.text().await?,
            Some("route") => route = field.text().await?,
            Some("file") => content = field.bytes().await?.to_vec(),
            _ => {}
        }
    }

    upload_file(&name, &route, content).await?;

    Ok(Json(DataResponde {
        message_status: "Archivo subido con éxito".to_string()
    }))
}
