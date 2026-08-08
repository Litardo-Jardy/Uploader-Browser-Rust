use tokio::fs; 
use dotenvy::dotenv;
use std::env;
use base64::{Engine, engine::general_purpose};
use crate::utils::path_validation::path_exists;
use crate::utils::path_validation::PathRequirement;
use crate::utils::name_path_validation::name_path_validation;

pub async fn add_folder(name: &str) -> Result<(), std::io::Error> {
    dotenv().ok();
    let base_dir = env::var("BASE_DIR").expect("Ruta no definida");
    name_path_validation(name).await?;
    let route = format!("{}{}", &base_dir, name);

    path_exists(&route, PathRequirement::MustNotExist, name).await?;
   
    fs::create_dir_all(&route).await?;

    let id = general_purpose::URL_SAFE_NO_PAD.encode(name);
    let metadata = format!("{{\"id\": \"{}\"}}", id);
    let metadata_path = format!("{}/.metadata.json", route);
    fs::write(&metadata_path, metadata).await?;
    
    Ok(())
}

