use actix_web::{web, HttpRequest, HttpResponse};
use bytes::{Bytes, BytesMut};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetClipboardReq {
    pub content: Option<String>,
    pub cliptype: Option<i32>,
    pub onetime: Option<bool>,
    pub password: Option<String>,
    pub expire_date: Option<i64>,
}

pub async fn set_clipboard(item: web::Json<SetClipboardReq>) -> HttpResponse {
    HttpResponse::Ok().body("ok.")
}
