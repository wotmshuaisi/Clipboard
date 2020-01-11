use actix_cors::Cors;
use actix_files::NamedFile;
use actix_rt;
use actix_web::{web, App, HttpServer, Result};
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
const TEMP_PATH: &str = "tmp/";
const MINIO_PUBLIC_PATH: &str = "s3/api/";
const MINIO_URL_PREFIX: &str = "http://localhost:9001/api/";
const STORAGE_ACCESS_PREFIX: &str = "http://localhost:8000/api/storage/";

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./html/index.html")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    /* Variables */
    let env_log_path = utils::get_env("LOG_PATH", DEFAULT_LOG_PATH);
    let env_mongo_addr = utils::get_env("MONGO_ADDR", MONGO_ADDR);
    let env_app_salt = utils::get_env("APP_SALT", APP_SALT);
    let env_listen_addr = utils::get_env("LISTEN_ADDR", LISTEN_ADDR);
    let env_temp_path = utils::get_env("TEMP_PATH", TEMP_PATH);
    let env_minio_public_path = utils::get_env("MINIO_PUBLIC_PATH", MINIO_PUBLIC_PATH);
    let env_minio_url_prefix = utils::get_env("MINIO_URL_PREFIX", MINIO_URL_PREFIX);
    let env_storage_access_prefix = utils::get_env("STORAGE_ACCESS_PREFIX", STORAGE_ACCESS_PREFIX);
    let env_mode = |x: String| -> String {
        if &x == RELEASE_MODE {
            return x;
        }
        String::from(DEBUG_MODE)
    }(utils::get_env("MODE", DEBUG_MODE).to_uppercase());
    /* Initialization */
    let (_guard, logger) = initial_logger(&env_mode, &env_log_path);
    let mongo_client = initial_mongo(&env_mongo_addr);
    let model_handler = models::ModelHandler::new(models::ModelHandlerOptions {
        conn: mongo_client.clone(),
        key: env_app_salt,
        minio_public_path: env_minio_public_path,
        storage_access_prefix: env_storage_access_prefix,
    });
    /* Operations */
    info!(
        slog_scope::logger(),
        "starting service, listen address: [{}]", env_listen_addr
    );
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/p/{id:.*}", web::get().to(index))
            .service(actix_files::Files::new("/statics", "./html/statics").show_files_listing())
            .wrap(Cors::new().finish())
            .wrap(logging::Logging::new(logger.clone()))
            .data(api::HandlerState {
                model: model_handler.clone(),
                temp_path: env_temp_path.clone(),
                minio_storage_prefix: env_minio_url_prefix.clone(),
                proxy_client: actix_web::client::Client::new(),
            })
            .configure(api::set_api_router)
    })
    .bind(env_listen_addr)?
    .run()
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
