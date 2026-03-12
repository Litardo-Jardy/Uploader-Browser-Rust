use axum::{Router, routing::post, Json, http::StatusCode};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::models::claims::Claims;
use dotenvy::dotenv;
use std::env;

#[derive(Deserialize)]
struct LoginInput {
  user: String,
  pass: String,
}

#[derive(Serialize)]
struct LoginResponse{
  token: String,
}

pub fn routes() -> Router {
   Router::new()
       .route("/login", post(login))
}

async fn login( Json(body): Json<LoginInput> ) -> Result<Json<LoginResponse>, StatusCode> {
     
     dotenv().ok();
     let secret = env::var("SECRET").expect("Secret no definido");
     let user = env::var("USERR").expect("User no definido");
     let pass = env::var("PASS").expect("Pass no definida");
     println!("{}/{}", &user, &pass);
     if body.user != user || body.pass != pass {
        return Err(StatusCode::UNAUTHORIZED); 
     }

     let expiration = SystemTime::now()
         .duration_since(UNIX_EPOCH)
         .unwrap()
         .as_secs() as usize + (60 * 60 * 24);

     let claims = Claims {
       sub: body.user,
       exp: expiration,
     };

     let token = encode(
       &Header::default(),
       &claims,
       &EncodingKey::from_secret(&secret.as_bytes())
     ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

     Ok(Json(LoginResponse { token }))
}
