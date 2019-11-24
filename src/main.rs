use actix_web::{App, HttpServer};

use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson};
use mongodb::{Client, ThreadedClient};

mod api;
mod logging;
mod models;
mod utils;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_scope;
extern crate slog_term;

const DEFAULT_LOG_PATH: &str = "./log/";
const DEBUG_MODE: &str = "DEBUG";
const RELEASE_MODE: &str = "RELEASE";

fn main() {
    /* Variables */
    let env_mode = match std::env::var("MODE") {
        Ok(val) => {
            if val.to_uppercase() == "RELEASE" {
                String::from(RELEASE_MODE)
            } else {
                String::from(DEBUG_MODE)
            }
        }
        Err(_) => String::from(DEBUG_MODE),
    };
    let env_log_path = match std::env::var("LOG_PATH") {
        Ok(val) => val,
        Err(_) => String::from(DEFAULT_LOG_PATH),
    };
    /* Initialization */
    let (_guard, logger) = initial_logger(env_mode, env_log_path);
    let _ = models::new();
    initial_mongo();
    /* Operations */
    HttpServer::new(move || {
        App::new()
            .wrap(logging::Logging::new(logger.clone()))
            .configure(api::set_api_router)
    })
    .bind("0.0.0.0:8000")
    .unwrap()
    .run()
    .unwrap();
}

// set up global logger & http logger
fn initial_logger(mode: String, log_path: String) -> (slog_scope::GlobalLoggerGuard, slog::Logger) {
    let logger = utils::new_logger(log_path.clone() + "main.log", "main", false);
    let _guard = slog_scope::set_global_logger(logger.clone());

    match mode.as_str() {
        RELEASE_MODE => (
            _guard,
            utils::new_logger(log_path.clone() + "http.log", "http", false),
        ),
        _ => (
            _guard,
            utils::new_logger(log_path.clone() + "http.log", "http", true),
        ),
    }
}

fn initial_mongo() {
    let client = mongodb::Client::connect("localhost", 27017).unwrap();
    let test_db = client.db("test");
    // test connection
    test_db
        .create_collection("test_conn_by_clipboard", None)
        .unwrap();
    test_db.drop_collection("test_conn_by_clipboard").unwrap();
}
