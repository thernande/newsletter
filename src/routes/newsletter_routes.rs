use crate::controller::newsletter_controller;
use actix_web::web;

pub fn newsletter(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/newsletter").service(newsletter_controller::greet));
}
// let newsletter = web::scope("/newsletter").service(newsletter_controller::get_newsletter);
