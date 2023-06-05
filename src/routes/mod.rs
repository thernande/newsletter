mod newsletter_routes;
use actix_web::web;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").configure(newsletter_routes::newsletter));
}
