use tokio::fs::{ self };
use dotenvy::dotenv;
use std::env;
use serde::{ Serialize};
use crate::utils::name_path_validation::name_path_validation;

#[derive(Serialize)]
pub struct FileInfo {
  name: String,
  size: u64,
  extension: String,
  url: String,
  path: String,
}

pub async fn list_file(mut path: &str) -> Result<Vec<FileInfo>, std::io::Error> {
   dotenv().ok();
   let base_dir = env::var("BASE_DIR").expect("Ruta no definida");

   name_path_validation(path).await?;
   let route = if path == "*" {
      path = "";
      base_dir.to_string()
     } else {
        format!("{}{}", base_dir, path)};

   let mut files = fs::read_dir(&route).await?;
   let mut name_files = Vec::new();

   while let Some(file) = files.next_entry().await? {
     let metadata = file.metadata().await?;

     if metadata.is_file() {
         let file_name = file.file_name().to_string_lossy().to_string();
         let host = env::var("HOST").expect("Host no definido");
         let host_url = format!("{}{}/{}", &host, path, file_name);
         name_files.push(FileInfo { 
             name: file_name,
             size: metadata.len(),
             extension: file.path().extension().and_then(|e| e.to_str()).unwrap_or("").to_string(),
             url: host_url,
             path: path.to_string()
         });
     }
   }
   Ok(name_files)
}
