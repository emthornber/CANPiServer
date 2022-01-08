use actix_files as fs;
use crate::handlers::*;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(fs::Files::new("/static", "./canpi-web-app-ssr/static")
                .show_files_listing())
            .service(web::resource("/")
                .route(web::get().to(status_handler)))
            .service(web::resource("/config")
                .route(web::get().to(display_config)))
    );
}