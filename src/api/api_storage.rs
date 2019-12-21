use actix_web::{error, web, HttpResponse};
// use serde_derive::Deserialize;
// use serde_json::json;
// use std::time::SystemTime;

use actix_multipart::Multipart;
// use std::collections::HashMap;

use crate::api;
use crate::utils;
// use crate::models::{ClipboardModel, ClipboardType, CreateClipboard};

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
    println!("{:#?}", map);

    Ok(HttpResponse::Ok().body("ok"))
}
