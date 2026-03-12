use tokio::fs;
use crate::utils::path_validation::path_exists;
use crate::utils::path_validation::PathRequirement;
use crate::utils::name_path_validation::name_path_validation;
use crate::models::config::BASE_DIR;

pub async fn edit_folder(name: &str, new_name: &str) -> Result<(), std::io::Error> {
     name_path_validation(name).await?;
      
     let route = format!("{}/{}", BASE_DIR, name);
     path_exists(&route, PathRequirement::MustExist).await?;

     let new_rute = format!("{}/{}", BASE_DIR, new_name); 

     fs::rename(&route, &new_rute).await?;
     Ok(())
}
