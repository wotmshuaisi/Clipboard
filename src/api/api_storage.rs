use actix_multipart::Multipart;
use actix_web::{error, web, Error, HttpMessage, HttpRequest, HttpResponse};
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
    let is_attachments = req.path().contains("attachments");
    let map = utils::multipart_processor(
        &h.temp_path,
        "[]file",
        payload,
        match is_attachments {
            true => 4,
            false => 0,
        },
    )
    .await?;
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
            if is_attachments && c.attachments_url.is_some() && c.attachments_url.unwrap().len() > 0
            {
                return Err(error::ErrorBadRequest(
                    "can not upload more files to this clipboard",
                ));
            }
            if req.cookie("token").is_none() || req.cookie("token").unwrap().value() != &c.token {
                return Err(error::ErrorBadRequest(
                    "you don't have permission to edit the resource.",
                ));
            }
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
            Ok(link) => links.push((String::from(file_name_path[0]), link)),
            Err(_) => {
                return Err(error::ErrorInternalServerError(""));
            }
        }
    }

    Ok(HttpResponse::Created().json(json!({ "links": links })))
}

pub async fn get_clipboard_file(
    req: web::HttpRequest,
    h: web::Data<api::HandlerState>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, error::Error> {
    if req.cookie("token").is_none() {
        return Err(error::ErrorForbidden(
            "you don't have permission to access the resource.",
        ));
    }
    match h.model.retrieve_clipboard(GetClipboard {
        id: path.0.clone(),
        expire_check: true,
        is_set: true,
    }) {
        Ok(c) => {
            if c.is_some() {
                let c = c.unwrap();
                if c.is_lock && req.cookie("token").is_none()
                    || req.cookie("token").unwrap().value() != c.token
                {
                    return Err(error::ErrorForbidden(
                        "you don't have permission to access the resource.",
                    ));
                }
            }
        }
        Err(_) => {
            return Err(error::ErrorInternalServerError(""));
        }
    };

    let mut client_res = h
        .proxy_client
        .get(h.minio_clipboard_url(&path.0, &path.1))
        .send()
        .await
        .map_err(Error::from)?;

    let mut res = HttpResponse::build(client_res.status());

    for (key, value) in client_res
        .headers()
        .iter()
        .filter(|(h, _)| *h != "connection")
    {
        res.header(key.clone(), value.clone());
    }

    Ok(res.body(client_res.body().await?))
}
