use crate::controller::newsletter_controller;
use actix_web::web;

pub fn newsletter(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/newsletter")
            .service(newsletter_controller::greet)
            .service(newsletter_controller::health_check)
            .service(newsletter_controller::subscribe),
    );
}
// let newsletter = web::scope("/newsletter").service(newsletter_controller::get_newsletter);
