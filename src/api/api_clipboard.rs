use actix_web::{error, http, web, HttpMessage, HttpRequest, HttpResponse};
use serde_derive::Deserialize;
use serde_json::json;
use std::time::SystemTime;

use crate::api;
use crate::models::{ClipboardModel, ClipboardType, GetClipboard, SetClipboard};

/* Structures */

#[derive(Debug, Default, Deserialize)]
pub struct SetClipboardReq {
    pub id: String,
    pub content: String,
    pub password: Option<String>,
    #[serde(default)]
    pub onetime: bool,
    #[serde(default = "default_clip_type")]
    pub cliptype: u8,
    #[serde(default = "default_expire_date")]
    pub expire_date: u64,
    pub attachments_url: Option<Vec<String>>,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct RetrieveReq {
    pub password: Option<String>,
}

/* Handlers */

pub async fn create_clipboard(
    h: web::Data<api::HandlerState>,
) -> Result<HttpResponse, error::Error> {
    match h.model.create_clipboard() {
        Ok(val) => Ok(HttpResponse::Created()
            .cookie(
                http::Cookie::build("token", val.1)
                    .path("/")
                    .secure(true)
                    .http_only(false)
                    .finish(),
            )
            .json(json!( {"id": val.0}))),
        Err(_) => Err(error::ErrorInternalServerError("")),
    }
}

pub async fn set_clipboard(
    req: HttpRequest,
    h: web::Data<api::HandlerState>,
    item: web::Json<SetClipboardReq>,
) -> Result<HttpResponse, error::Error> {
    if item.content.is_empty() {
        return Err(error::ErrorBadRequest("content can not be empty"));
    }
    if item.content.len() > 4900000 {
        return Err(error::ErrorBadRequest(
            "maximum content size is 4900000 bytes",
        ));
    }
    if SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 600 as u64
        > item.expire_date
    {
        return Err(error::ErrorBadRequest(
            "expiredate must greater than 10 minutes",
        ));
    }
    let clip_type = match ClipboardType::from_u8(item.cliptype) {
        Some(val) => val,
        _ => {
            return Err(error::ErrorBadRequest("wrong cliptype"));
        }
    };

    match h.model.retrieve_clipboard(GetClipboard {
        id: String::from(&item.id),
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
            if req.cookie("token").is_none() || req.cookie("token").unwrap().value() != &c.token {
                return Err(error::ErrorBadRequest(
                    "you don't have permission to edit this clipboard.",
                ));
            }
        }
        Err(_) => {
            return Err(error::ErrorInternalServerError(""));
        }
    };

    match h.model.set_clipboard(SetClipboard {
        id: String::from(&item.id),
        clip_content: String::from(&item.content),
        clip_type: clip_type,
        clip_onetime: item.onetime,
        is_lock: match item.password.is_none() {
            true => false,
            false => true,
        },
        password: match &item.password {
            Some(val) => Some(String::from(val)),
            None => None,
        },
        expire_date: item.expire_date as i64,
        attachments_url: match &item.attachments_url {
            Some(val) => Some(val.to_vec()),
            None => None,
        },
    }) {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Err(error::ErrorInternalServerError("")),
    }
}

pub async fn retrieve_clipboard(
    h: web::Data<api::HandlerState>,
    path: web::Path<(String,)>,
    query: web::Query<RetrieveReq>,
) -> Result<HttpResponse, error::Error> {
    match h.model.retrieve_clipboard(GetClipboard {
        id: path.0.to_lowercase(),
        expire_check: true,
        is_set: true,
    }) {
        Ok(val) => match val {
            Some(c) => {
                // password
                if c.is_lock {
                    // empty password
                    match &query.password {
                        Some(val) => {
                            // password validator
                            if !c.password_valid(&val) {
                                return Err(error::ErrorBadRequest("wrong password."));
                            }
                        }
                        _ => {
                            return Err(error::ErrorBadRequest(
                                "password is required for this clipboard.",
                            ));
                        }
                    }
                }
                // remove item if it's one-time clipboard
                if c.clip_onetime {
                    h.model.destroy_clipboard(&c.id).unwrap();
                }
                Ok(HttpResponse::Ok().json(c))
            }
            None => Err(error::ErrorNotFound("no resource has been found.")),
        },
        Err(_) => Err(error::ErrorInternalServerError("")),
    }
}

pub async fn islock_clipboard(
    h: web::Data<api::HandlerState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, error::Error> {
    match h.model.retrieve_clipboard(GetClipboard {
        id: path.0.to_lowercase(),
        expire_check: true,
        is_set: true,
    }) {
        Ok(val) => match val {
            Some(c) => Ok(HttpResponse::Ok().json(json!({"is_lock": c.is_lock}))),
            None => Err(error::ErrorNotFound("no resource has been found.")),
        },
        Err(_) => Err(error::ErrorInternalServerError("")),
    }
}

/* Functions */

// default expire date for clipboard is one day
fn default_expire_date() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + (3600 * 24)
}

fn default_clip_type() -> u8 {
    1
}

/* Test Functions */

#[test]
fn set_clipboard_req_test() {
    let data = r#"{"content":"test"}"#;
    let req: SetClipboardReq = serde_json::from_str(data).unwrap();
    assert_eq!(req.cliptype, 0);
    assert_eq!(req.content.as_str(), "test");
    assert_eq!(
        req.expire_date
            > SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        true
    );
    assert_eq!(req.onetime, false);
    assert_eq!(req.password, None);
}
