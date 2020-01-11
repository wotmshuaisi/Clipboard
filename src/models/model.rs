use mongodb::ThreadedClient;
use std::clone::Clone;
use std::error::Error;

use crate::models;

/* Interface & Structures */

pub trait ClipboardModel {
    fn create_clipboard(&self) -> Result<(String, String), Box<dyn Error>>;
    fn set_clipboard(&self, c: models::SetClipboard) -> Result<(), Box<dyn Error>>;
    fn destroy_clipboard(&self, id: &str, files: bool) -> Result<(), Box<dyn Error>>;
    fn retrieve_clipboard(
        &self,
        opt: models::GetClipboard,
    ) -> Result<Option<models::Clipboard>, Box<dyn Error>>;
}

pub trait StorageModel {
    fn save_to_minio(
        &self,
        source: &str,
        folder: &str,
        file_name: &str,
    ) -> Result<String, Box<dyn Error>>;
    fn remove_minio_folder(&self, path: &str) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone, Debug)]
pub struct ModelHandler {
    pub db: mongodb::db::Database,
    pub logger: slog::Logger,
    pub key: Vec<u8>,
    pub minio_public_path: String,
    pub storage_access_prefix: String,
}

pub struct ModelHandlerOptions {
    pub conn: mongodb::Client,
    pub key: String,
    pub minio_public_path: String,
    pub storage_access_prefix: String,
}

/* Implement */

impl ModelHandler {
    pub fn new(opt: ModelHandlerOptions) -> ModelHandler {
        use openssl::hash::{hash, MessageDigest};
        ModelHandler {
            db: opt.conn.db("clipboard"),
            logger: slog_scope::logger(),
            key: hash(MessageDigest::sha256(), opt.key.as_bytes())
                .unwrap()
                .to_vec(),
            minio_public_path: opt.minio_public_path,
            storage_access_prefix: opt.storage_access_prefix,
        }
    }
}

impl ModelHandler {
    pub fn err_log(&self, model_func_name: &str, index: i8, err: &str) {
        error!(
            self.logger,
            "{} [MD_FN:{} I:{}]", err, model_func_name, index,
        )
    }

    pub fn warn_log(&self, model_func_name: &str, index: i8) {
        warn!(self.logger, "[MD_FN:{} I:{}]", model_func_name, index,)
    }
}

/* Test code */

#[allow(dead_code)]
pub fn initial_test_handler() -> ModelHandler {
    use crate::utils;
    let _guard = slog_scope::set_global_logger(utils::new_logger(
        String::from("./log/") + "test.log",
        "test",
        false,
    ));
    let client = mongodb::Client::with_uri("mongodb://127.0.0.1:27017/").unwrap();
    models::ModelHandler::new(models::ModelHandlerOptions {
        conn: client,
        key: String::from("test_salt"),
        minio_public_path: String::from(""),
        storage_access_prefix: String::from(""),
    })
}
