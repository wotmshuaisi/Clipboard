use actix_web::Error;
use actix_web::{HttpRequest, HttpResponse};
use futures::future::{ok, Future};

pub fn set_clipboard(req: HttpRequest) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    println!("{}", req.headers().get("content-type").unwrap() == "test");
    Box::new(ok::<_, Error>(
        HttpResponse::Ok()
            .content_type("text/html")
            .body("<h1>test</h1>"),
    ))
}