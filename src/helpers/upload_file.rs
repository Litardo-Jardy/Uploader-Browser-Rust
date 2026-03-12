use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::models::config::BASE_DIR;
use crate::utils::path_validation::path_exists;
use crate::utils::path_validation::PathRequirement;
use crate::utils::name_path_validation::name_path_validation;

pub async fn upload_file(
             _name: &str, 
             _routes: &str, 
             _file: Vec<u8>) -> Result<(), std::io::Error> {
    
    name_path_validation(_name).await?;
    let mut _route = "";
    if _routes == "*" {
      _route = ""
    } else {
      _route = _routes;
    }
    let route_file = format!("{}{}", BASE_DIR, _route);
 
    path_exists(&route_file, PathRequirement::MustExist).await?;
    let route_file = format!("{}{}/{}", BASE_DIR, _route, _name);
    path_exists(&route_file, PathRequirement::MustNotExist).await?;

    let mut file = fs::File::create(&route_file).await?;
    file.write_all(&_file).await?;

    Ok(())
}
