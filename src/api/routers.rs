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

*/

#[derive(Debug)]
pub struct HandlerState {
    pub model: models::ModelHandler,
}

pub fn set_api_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/clipboard")
                    .route(web::get().to(|| web::HttpResponse::NoContent()))
                    .route(web::post().to(api::set_clipboard)),
            )
            .service(
                web::resource("/clipboard/{id}").route(web::get().to(api::retrieve_clipboard)),
            ),
    );
}
