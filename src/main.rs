use actix_web::{App, HttpServer};
use mongodb::db::ThreadedDatabase;
use mongodb::ThreadedClient;

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
const MONGO_ADDR: &str = "mongodb://127.0.0.1:27017/admin";

fn main() {
    /* Variables */
    let env_log_path = utils::get_env("LOG_PATH", DEFAULT_LOG_PATH);
    let env_mongo_addr = utils::get_env("MONGO_ADDR", MONGO_ADDR);
    let env_mode = |x: String| -> String {
        if &x == RELEASE_MODE {
            x
        } else {
            String::from(DEBUG_MODE)
        }
    }(utils::get_env("MODE", DEBUG_MODE).to_uppercase());
    /* Initialization */
    let (_guard, logger) = initial_logger(env_mode, env_log_path);
    let mongo_client = initial_mongo(env_mongo_addr);
    let _ = models::new(mongo_client.clone());
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

fn initial_mongo(mongo_addr: String) -> mongodb::Client {
    let client = mongodb::Client::with_uri(mongo_addr.as_ref()).unwrap();
    // test connection
    match client.db("clipboard").version() {
        Ok(version) => {
            info!(
                slog_scope::logger(),
                "mongodb version: {}.{}.{}", version.major, version.minor, version.patch
            );
        }
        Err(err) => {
            panic!("[{}] - initial_mongo", err);
        }
    };
    client
}
