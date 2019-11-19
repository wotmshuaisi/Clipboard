use actix_web::web;

use crate::api;

pub fn set_api_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/clipboard").route(web::post().to_async(api::set_clipboard))),
    );
}
