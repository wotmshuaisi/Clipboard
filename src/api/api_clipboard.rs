use actix_web::{web, HttpRequest, HttpResponse};
use bytes::{Bytes, BytesMut};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value::{Bool, Number, String};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct SetClipboardReq {
    // pub content: Option<String>,
    // #[serde(default = 1)]
    // pub cliptype: Number,
    pub onetime: Option<bool>,
    // pub password: Option<String>,
    pub expire_date: Option<i64>,
}

impl std::default::Default for SetClipboardReq {
    fn default() -> SetClipboardReq {
        SetClipboardReq {
            onetime: Some(true),
            expire_date: Some(0),
        }
    }
}

pub async fn set_clipboard(item: web::Json<SetClipboardReq>) -> HttpResponse {
    HttpResponse::Ok().body("ok.")
}
