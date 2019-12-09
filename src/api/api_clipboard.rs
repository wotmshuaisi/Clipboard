use actix_web::{HttpRequest, HttpResponse};

pub async fn set_clipboard(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("ok.")
}
