use tokio::fs;
use tokio::io::AsyncWriteExt;
use dotenvy::dotenv;
use std::env;
use crate::utils::path_validation::path_exists;
use crate::utils::path_validation::PathRequirement;
use crate::utils::name_path_validation::name_path_validation;

pub async fn upload_file(
             _name: &str, 
             _routes: &str, 
             _file: Vec<u8>) -> Result<(), std::io::Error> {
    
    dotenv().ok();
    let base_dir = env::var("BASE_DIR").expect("Ruta no definida");

    name_path_validation(_name).await?;
    let mut _route = "";
    if _routes == "*" {
      _route = ""
    } else {
      _route = _routes;
    }
    let route_file = format!("{}{}", base_dir, _route);
 
    path_exists(&route_file, PathRequirement::MustExist, _name).await?;
    let route_file = format!("{}{}/{}", base_dir, _route, _name);
    path_exists(&route_file, PathRequirement::MustNotExist, _name).await?;

    let mut file = fs::File::create(&route_file).await?;
    file.write_all(&_file).await?;

    Ok(())
}
