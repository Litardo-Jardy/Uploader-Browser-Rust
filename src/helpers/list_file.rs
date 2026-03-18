use tokio::fs::{self, metadata};
use dotenvy::dotenv;
use std::env;
use crate::utils::name_path_validation::name_path_validation;

struct FileInfo {
  name: String,
  size: u64,
  extension: String,
  url: String
}

pub async fn list_file(path: &str) -> Result<Vec<String>, std::io::Error> {
   dotenv().ok();
   let base_dir = env::var("BASE_DIR").expect("Ruta no definida");

   name_path_validation(path).await?;
   let route = if path == "*" {
      base_dir.to_string()
     } else {
        format!("{}/{}", base_dir, path)};

   let mut files = fs::read(&route).await?;
   let mut name_files = Vec::new();

   while let Some(file) = files.next_entry().await? {
     let metadata = file.metadata().await?;

     if metadata.is_file() {
         let host = env::var("HOST").expect("Host no definido");
         let hostUrl = format!("{}{}", &host, path);
         name_files.push(FileInfo { 
             name: file.file_name().to_string_lossy().to_string(),
             size: metadata.len(),
             extension: file.extension(),
             url: &hostUrl 
         });
     }
   }
   Ok(name_files)
}
