use crate::model::newsletter;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;

use sqlx::PgPool;
use uuid::Uuid;

#[get("/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {:?}!", &name)
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
    request_id = %Uuid::new_v4(),
    subscriber_email = %form.email,
    subscriber_name = %form.name
    )
    )]
#[post("/subscriptions")]
async fn subscribe(
    _form: web::Form<newsletter::FormData>,
    _pool: web::Data<PgPool>,
) -> HttpResponse {
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().body("se guardo el correo correctamente"),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

/*
#[delete("/deleteEmail")]
async fn deleteEmail(
    _form: web::Form<newsletter::FormData>,
    _pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
        DELETE FROM subscriptions where email=$1 and name=$2
        "#,
        _form.email,
        _form.name
    )
    .execute(_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
*/
