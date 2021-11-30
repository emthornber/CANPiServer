use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/config")
            .route("/", web::post().to(new_config)));
}