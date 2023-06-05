use crate::model::newsletter;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {:?}!", &name)
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<newsletter::FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
