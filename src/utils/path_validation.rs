use std::{io, path::Path};

pub enum PathRequirement {
    MustExist,
    MustNotExist,
}

pub async fn path_exists(route: &str, requirement: PathRequirement, name: &str) -> Result<(), io::Error> {

    let exists = Path::new(route).exists();

    match requirement {
        PathRequirement::MustExist if !exists => {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "La ruta espesificada no existe"
            ))
        }

        PathRequirement::MustNotExist if exists => {
            Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Ya existe un archivo '{}' en el directorio actual", name),
            ))
        }

        _ => Ok(())
    }
}
