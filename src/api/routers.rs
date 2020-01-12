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

pub struct HandlerState {
    pub model: models::ModelHandler,
    pub temp_path: String,
    pub minio_storage_prefix: String,
    pub proxy_client: actix_web::client::Client,
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
                    .route("", web::put().to(api::set_clipboard)),
            )
            .service(
                web::scope("/storage")
                    .route("", web::get().to(|| web::HttpResponse::MethodNotAllowed()))
                    .service(
                        web::scope("/clipboard")
                            .route("", web::get().to(|| web::HttpResponse::MethodNotAllowed()))
                            .route(
                                "/{taskid}/{filename}",
                                web::get().to(api::get_clipboard_file),
                            )
                            .route("/images", web::post().to(api::upload_clipboard_files))
                            .route("/attachments", web::post().to(api::upload_clipboard_files)),
                    ),
            ),
    );
}

impl HandlerState {
    pub fn minio_clipboard_url(&self, id: &str, file: &str) -> String {
        return format!(
            "{}clipboard/{}/{}",
            String::from(&self.minio_storage_prefix),
            id,
            file
        );
    }
}
