use actix_web::{error, web, HttpResponse};

use actix_multipart::Multipart;

use crate::api;
use crate::models::{ClipboardModel, GetClipboard};
use crate::utils;

/* Structures */

/* Handlers */

/*

    Todo:
        - util function
            - hashmap [done]
                save fieldname - "value" [done]
                save fieldname - "filename|temparory path" [done]
        - save to minio by taskid
*/

pub async fn upload_to_storage(
    h: web::Data<api::HandlerState>,
    payload: Multipart,
) -> Result<HttpResponse, error::Error> {
    let map = utils::multipart_processor(h.temp_path.clone(), "[]file", payload).await?;
    let task_id = match map.get("task_id") {
        Some(val) => val,
        None => {
            return Err(error::ErrorBadRequest("[task_id] is required"));
        }
    };

    match h.model.retrieve_clipboard(GetClipboard {
        id: task_id.to_string(),
        expire_check: false,
        is_set: false,
    }) {
        Ok(c) => {
            if c.is_none() {
                return Err(error::ErrorNotFound("no resource has been found."));
            }
            if c.unwrap().is_set {
                return Err(error::ErrorBadRequest(
                    "this clipboard has already been setup.",
                ));
            }
        }
        Err(_) => {
            return Err(error::ErrorInternalServerError(""));
        }
    };

    println!("{:#?}", map);
    Ok(HttpResponse::Ok().body("ok"))
}
