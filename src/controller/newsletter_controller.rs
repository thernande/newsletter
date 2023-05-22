use actix_web::{get, web, Responder};

#[get("/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {:?}!", &name)
}
