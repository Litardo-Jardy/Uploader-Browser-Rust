use axum::{
  Router,
  routing::get, 
  Json
};
use serde::Serialize;

#[derive(Serialize)]
struct Saludo {
   nombre: String,
   ubicacion: String,
   mensaje: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/user", get(saludo))
}

async fn saludo() -> Json<Saludo> {
    let respuesta = Saludo {
       nombre: "Nicolas maduro".to_string(),
       ubicacion: "United States - New York".to_string(),
       mensaje: "Coño he la madre, diganle a Trump que me deje en paz".to_string(),
    };

    Json(respuesta)
}
