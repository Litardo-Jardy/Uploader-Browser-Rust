use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use async_trait::async_trait;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::claims::Claims;
use dotenvy::dotenv;
use std::env;

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
   
        dotenv().ok();
        let secret = env::var("SECRET").expect("Secret no definido");
        let auth_header = parts.headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

 
let data = decode::<Claims>(
    token,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::default()
).map_err(|_| {
        StatusCode::UNAUTHORIZED
})?;

        Ok(UsuarioAutenticado {
            usuario: data.claims.sub
        })

    }
}
