use tokio::fs;
use dotenvy::dotenv;
use std::env;
use serde::{ Serialize, Deserialize };
use crate::utils::name_path_validation::name_path_validation;

#[derive(Deserialize)]
struct FolderMetadata {
    id: String,}

#[derive(Serialize)]
pub struct FolderInfo {
    id: String,
    name: String,
    size: u64,
    element_count: u32,
    path: String}

pub async fn list_folders(path: &str) -> Result<Vec<FolderInfo>, std::io::Error> {
    dotenv().ok();
    let base_dir = env::var("BASE_DIR").expect("Ruta no definida");

    name_path_validation(path).await?;
    let route = if path == "*" {
        base_dir.to_string()
    } else {
        format!("{}/{}", base_dir, path)
    };

    let mut folders = fs::read_dir(&route).await?;
    let mut name_folders: Vec<FolderInfo> = Vec::new();

    while let Some(folder) = folders.next_entry().await? {
        let metadata = folder.metadata().await?;

        if metadata.is_dir() {
            let nombre = folder.file_name().to_string_lossy().to_string();

            let metadata_path = folder.path().join(".metadata.json");
            let id = if let Ok(content) = fs::read_to_string(&metadata_path).await {
                     serde_json::from_str::<FolderMetadata>(&content)
                     .map(|m| m.id)
                     .unwrap_or_default()
            } else {
              String::new()};

            let mut size: u64 = 0;
            let mut element_count: u32 = 0;
            let mut entradas = fs::read_dir(folder.path()).await?;

            while let Some(entrada) = entradas.next_entry().await? {
                let meta = entrada.metadata().await?;
                    size += meta.len();
                    element_count += 1;
            }

            name_folders.push(FolderInfo { 
                id,
                name: nombre, 
                size,
                element_count,
                path: path.to_string()
            });
        }
    }

    Ok(name_folders)
}
