use tokio::fs;
use std::io;
use crate::models::config:: BASE_DIR;

pub async fn list_folders(path: &str) -> Result<Vec<String>, std::io::Error> {
   if path.contains("..") {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "path inválido"));}

   let route = if path.is_empty() {
      BASE_DIR.to_string()
     } else {
        format!("{}/{}", BASE_DIR, path)};

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
