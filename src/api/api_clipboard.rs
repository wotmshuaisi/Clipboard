use actix_web::Error;
use actix_web::{HttpRequest, HttpResponse};
use futures::future::{ok, Future};
use std::thread::sleep;

pub fn set_clipboard(req: HttpRequest) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(ok::<_, Error>(
        HttpResponse::Ok()
            .content_type("text/html")
            .body("<h1>test</h1>"),
    ))
}
