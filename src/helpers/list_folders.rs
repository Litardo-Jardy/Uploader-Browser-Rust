use tokio::fs;
use dotenvy::dotenv;
use std::env;
use crate::utils::name_path_validation::name_path_validation;

pub async fn list_folders(path: &str) -> Result<Vec<String>, std::io::Error> {
   dotenv().ok();
   let base_dir = env::var("BASE_DIR").expect("Ruta no definida");

   name_path_validation(path).await?;
   let route = if path == "*" {
      base_dir.to_string()
     } else {
        format!("{}/{}", base_dir, path)};

   let mut folders = fs::read_dir(&route).await?;
   let mut name_folders = Vec::new();

   while let Some(folder) = folders.next_entry().await? {
     let metadata = folder.metadata().await?;

     if metadata.is_dir() {
       let nombre = folder.file_name()
           .to_string_lossy()
           .to_string();
       name_folders.push(nombre);
     }
   }
   Ok(name_folders)
}
