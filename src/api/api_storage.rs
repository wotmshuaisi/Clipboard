use actix_multipart::Multipart;
use actix_web::{error, web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

use crate::api;
use crate::models::{ClipboardModel, GetClipboard, StorageModel};
use crate::utils;

/* Structures */

/* Handlers */

pub async fn upload_clipboard_files(
    req: HttpRequest,
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
    let files = match map.get("[]file") {
        Some(val) => val,
        None => {
            return Err(error::ErrorBadRequest("[[]file] is required"));
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
            let c = c.unwrap();
            if c.is_set {
                return Err(error::ErrorBadRequest(
                    "this clipboard has already been setup.",
                ));
            }
            // if req.cookie("token").is_none() || req.cookie("token").unwrap().value() != &c.token {
            //     return Err(error::ErrorBadRequest(
            //         "you don't have permission to edit this clipboard.",
            //     ));
            // }
        }
        Err(_) => {
            return Err(error::ErrorInternalServerError(""));
        }
    };

    let mut links: Vec<(String, String)> = Vec::new();

    let files: Vec<_> = files.split("||").collect();
    for file in files {
        if file == "" {
            break;
        }
        let file_name_path: Vec<_> = file.split("|").collect();
        if file_name_path.len() != 2 {
            continue;
        }
        match h.model.save_to_minio(
            file_name_path[1],
            &(String::from("clipboard/") + task_id),
            file_name_path[0],
        ) {
            Ok(link) => links.insert(0, (String::from(file_name_path[0]), link)),
            Err(_) => {
                return Err(error::ErrorInternalServerError(""));
            }
        }
    }

    Ok(HttpResponse::Created().json(json!({ "links": links })))
}

// pub async fn get_clipboard_file(
//     h: web::Data<api::HandlerState>,
//     path: web::Path<(String, String)>,
// ) -> Result<HttpResponse, error::Error> {
// }
