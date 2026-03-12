use tokio::fs;
use crate::utils::path_validation::path_exists;
use crate::utils::path_validation::PathRequirement;
use crate::utils::name_path_validation::name_path_validation;
use dotenvy::dotenv;
use std::env;

pub async fn edit_folder(name: &str, new_name: &str) -> Result<(), std::io::Error> {
     dotenv().ok();
     let base_dir = env::var("BASE_DIR").expect("Ruta no definida");

     name_path_validation(name).await?;
      
     let route = format!("{}/{}", &base_dir, name);
     path_exists(&route, PathRequirement::MustExist).await?;

     let new_rute = format!("{}/{}", &base_dir, new_name); 

     fs::rename(&route, &new_rute).await?;
     Ok(())
}
