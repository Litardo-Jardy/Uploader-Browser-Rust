use tokio::fs; 
use std::io;
use crate::models::config::BASE_DIR;

pub async fn add_folder(name: &str) -> Result<(), std::io::Error> {
    match name {
      n if n.len() <= 2 => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Nombre muy corto")),
      n if n.starts_with('.') => return Err(io::Error::new(io::ErrorKind::InvalidInput, "El nombre de la carpeta no puede iniciar con '.'")),
      _ => {}
    } 

    let rute = format!("{}/{}", BASE_DIR, name);
   
    fs::create_dir_all(rute).await?;
    Ok(())
}

