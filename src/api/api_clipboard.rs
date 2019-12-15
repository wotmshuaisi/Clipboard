use actix_web::{error, web, HttpResponse};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::api;
use crate::models::{ClipboardModel, ClipboardType, CreateClipboard};

/* Structures */

#[derive(Debug, Default, Deserialize)]
pub struct SetClipboardReq {
    pub content: String,
    pub password: Option<String>,
    #[serde(default)]
    pub onetime: bool,
    #[serde(default = "default_clip_type")]
    pub cliptype: u8,
    #[serde(default = "default_expire_date")]
    pub expire_date: u64,
}

pub async fn set_clipboard(
    h: web::Data<api::HandlerState>,
    item: web::Json<SetClipboardReq>,
) -> Result<HttpResponse, error::Error> {
    if item.content.is_empty() {
        return Err(error::ErrorBadRequest("content can not be empty"));
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

    match h.model.create_clipboard(CreateClipboard {
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
    }) {
        Ok(id) => {
            let mut map = HashMap::new();
            map.insert("id", id);
            Ok(HttpResponse::Ok().json(map))
        }
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
