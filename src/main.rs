use actix_rt;
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
const APP_SALT: &str = "saltforbcrypt";
const LISTEN_ADDR: &str = "0.0.0.0:8000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    /* Variables */
    let env_log_path = utils::get_env("LOG_PATH", DEFAULT_LOG_PATH);
    let env_mongo_addr = utils::get_env("MONGO_ADDR", MONGO_ADDR);
    let env_app_salt = utils::get_env("APP_SALT", APP_SALT);
    let env_listen_addr = utils::get_env("LISTEN_ADDR", LISTEN_ADDR);
    let env_mode = |x: String| -> String {
        if &x == RELEASE_MODE {
            return x;
        }
        String::from(DEBUG_MODE)
    }(utils::get_env("MODE", DEBUG_MODE).to_uppercase());
    /* Initialization */
    let (_guard, logger) = initial_logger(&env_mode, &env_log_path);
    let mongo_client = initial_mongo(&env_mongo_addr);
    let _models: models::ModelHandler = models::ClipboardModel::new(models::ModelHandlerOptions {
        conn: mongo_client.clone(),
        key: env_app_salt,
    });
    /* Operations */
    info!(
        slog_scope::logger(),
        "starting service, listen address: [{}]", env_listen_addr
    );
    HttpServer::new(move || {
        App::new()
            .wrap(logging::Logging::new(logger.clone()))
            .configure(api::set_api_router)
    })
    .bind(env_listen_addr)?
    .start()
    .await
}

// set up global logger & http logger
fn initial_logger(mode: &str, log_path: &str) -> (slog_scope::GlobalLoggerGuard, slog::Logger) {
    let _guard = slog_scope::set_global_logger(utils::new_logger(
        String::from(log_path) + "main.log",
        "main",
        false,
    ));

    match mode {
        RELEASE_MODE => (
            _guard,
            utils::new_logger(String::from(log_path) + "http.log", "http", true),
        ),
        _ => (
            _guard,
            utils::new_logger(String::from(log_path) + "http.log", "http", false),
        ),
    }
}

fn initial_mongo(mongo_addr: &str) -> mongodb::Client {
    let client = mongodb::Client::with_uri(mongo_addr).unwrap();
    // test connection
    match client.db("clipboard").version() {
        Ok(version) => {
            info!(
                slog_scope::logger(),
                "mongodb version: {}.{}.{}", version.major, version.minor, version.patch
            );
        }
        Err(err) => {
            panic!("[initial_mongo] {}", err);
        }
    };
    client
}
