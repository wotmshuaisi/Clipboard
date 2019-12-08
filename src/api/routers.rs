use actix_web::web;

use crate::api;

/*

Todo List:

    1. use async on function level
    2. get parameters from json
    3. error handling in api level
    4. finished clipboard creating interface
    5. finished clipboard retrieving interface
        1. password validation
        2. destory one-time or expired clipboards

*/

pub fn set_api_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/clipboard").route(web::post().to_async(api::set_clipboard))),
    );
}
