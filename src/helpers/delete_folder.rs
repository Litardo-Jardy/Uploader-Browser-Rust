use tokio::fs;
use std::io;
use crate::models::config::BASE_DIR;
use crate::utils::path_validation::path_exists;
use crate::utils::path_validation::PathRequirement;

pub async fn delete_folder(name: &str) -> Result<(), std::io::Error> {

    if name.contains("..") { return Err(io::Error::new(io::ErrorKind::InvalidInput, "Nombre inválido"));}

    if name.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "El nombre no puede estar vacío"));}

    let route = format!("{}/{}", BASE_DIR, name);
   
    path_exists(&route, PathRequirement::MustExist).await?;

    fs::remove_dir_all(&route).await?;
    Ok(())
}
