use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use async_trait::async_trait;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::claims::Claims;
use crate::models::config::SECRET;

pub struct UsuarioAutenticado {
    pub usuario: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for UsuarioAutenticado
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, StatusCode> {

        let auth_header = parts.headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

 
let data = decode::<Claims>(
    token,
    &DecodingKey::from_secret(SECRET.as_bytes()),
    &Validation::default()
).map_err(|e| {
    println!("Error decodificando token: {}", e);  // ver el error exacto
    StatusCode::UNAUTHORIZED
})?;

        Ok(UsuarioAutenticado {
            usuario: data.claims.sub
        })

    }
}
