use actix_web::web;

use crate::api;
use crate::models;

/*

Todo List:

    1. use async on function level [done]
    2. get parameters from json [done]
    3. error handling in api level [done]
    4. finished clipboard creating interface [done]
    5. finished clipboard retrieving interface [done]
        1. password validation [done]
        2. destory one-time or expired clipboards [done]
    6. interface for examine locked clipboard

*/

#[derive(Debug)]
pub struct HandlerState {
    pub model: models::ModelHandler,
    pub temp_path: String,
}

pub fn set_api_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/clipboard")
                    .route("", web::get().to(|| web::HttpResponse::MethodNotAllowed()))
                    .service(
                        web::scope("/{id}")
                            .route("", web::get().to(api::retrieve_clipboard))
                            .route("/is_lock", web::get().to(api::islock_clipboard)),
                    )
                    .route("", web::post().to(api::create_clipboard))
                    .data(web::JsonConfig::default().limit(6000000)) // maximum body size is 6mb
                    .route("", web::put().to(api::set_clipboard)),
            )
            .service(
                web::scope("/storage")
                    .route("", web::get().to(|| web::HttpResponse::MethodNotAllowed()))
                    .route("/clipboard", web::post().to(api::upload_clipboard_files)),
            ),
    );
}
