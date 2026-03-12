use std::io;

pub async fn name_path_validation(name: &str) -> Result<(), io::Error> {
    if name.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "El nombre no puede estar vacío"));}

    if name.len() > 100 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "El nombre no puede exceder 100 caracteres"));}

    if name.starts_with('.') {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "El nombre no puede iniciar con '.'"));}

    if name.contains("..") {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "El nombre no puede contener '..'"));}

    if name.contains(' ') {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "El nombre no puede contener espacios"));}

    if name.contains('/') || name.contains('\\') {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "El nombre no puede contener '/' ni '\\'"));}

    Ok(())
}
