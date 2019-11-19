use actix_web::{App, HttpServer};

mod api;

fn main() {
    HttpServer::new(move || App::new().configure(api::set_api_router))
        .bind("0.0.0.0:8000")
        .unwrap()
        .run()
        .unwrap();
}
