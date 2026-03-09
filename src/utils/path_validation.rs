use std::{io, path::Path};

pub enum PathRequirement {
    MustExist,
    MustNotExist,
}

pub async fn path_exists(route: &str, requirement: PathRequirement) -> Result<(), io::Error> {

    let exists = Path::new(route).exists();

    match requirement {
        PathRequirement::MustExist if !exists => {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("La ruta {} no existe", route),
            ))
        }

        PathRequirement::MustNotExist if exists => {
            Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("La ruta {} ya existe", route),
            ))
        }

        _ => Ok(())
    }
}
