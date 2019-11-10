use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

fn index() -> impl Responder {
    return HttpResponse::Ok().body("hello world!");
}

fn index2() -> impl Responder {
    return HttpResponse::Ok().body("hello world again!");
}

#[get("/hello")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("hello world third time!")
}

fn get() -> impl Responder {
    HttpResponse::Ok().body("/api/get")
}

fn main() {
    let mut listen_fd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service({
                web::scope("/api").route("/get", web::to(get)).route(
                    "/closures",
                    web::to(|| HttpResponse::Ok().body("response from closures")),
                )
            })
            .service(index3)
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    });

    server = if let Some(l) = listen_fd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("localhost:8000").unwrap()
    };

    server.run().unwrap();
}
