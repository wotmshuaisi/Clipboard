use actix_web::{HttpRequest, HttpResponse};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct set_clipboard_req {}

pub async fn set_clipboard(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("ok.")
}
