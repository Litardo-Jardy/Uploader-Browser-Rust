use tokio::fs;
use crate::models::config::BASE_DIR;
use crate::utils::path_validation::path_exists;
use crate::utils::path_validation::PathRequirement;
use crate::utils::name_path_validation::name_path_validation;

pub async fn delete_folder(name: &str) -> Result<(), std::io::Error> {
    name_path_validation(name).await?;
    let route = format!("{}/{}", BASE_DIR, name);
   
    path_exists(&route, PathRequirement::MustExist).await?;

    fs::remove_dir_all(&route).await?;
    Ok(())
}
