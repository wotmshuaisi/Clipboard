use actix_web::{App, HttpServer};

mod api;
mod models;

fn main() {
    let _ = models::new();

    HttpServer::new(move || App::new().configure(api::set_api_router))
        .bind("0.0.0.0:8000")
        .unwrap()
        .run()
        .unwrap();
}
