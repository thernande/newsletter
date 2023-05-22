use actix_web::{get, Responder, web};

#[get("/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}