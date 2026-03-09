use tokio::fs;
use std::io;
use crate::utils::path_validation::path_exists;
use crate::utils::path_validation::PathRequirement;
use crate::models::config::BASE_DIR;

pub async fn edit_folder(name: &str, new_name: &str) -> Result<(), std::io::Error> {
  
     match new_name {
      n if n.len() <= 2 => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Nombre muy corto")),
      n if n.starts_with('.') => return Err(io::Error::new(io::ErrorKind::InvalidInput, "El nombre de la carpeta no puede iniciar con '.'")),
      _ => {}
    } 
      
     let route = format!("{}/{}", BASE_DIR, name);
     path_exists(&route, PathRequirement::MustExist).await?;

     let new_rute = format!("{}/{}", BASE_DIR, new_name); 

     fs::rename(&route, &new_rute).await?;
     Ok(())
}
