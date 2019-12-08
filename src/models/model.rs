use std::error::Error;

use crate::models;

extern crate serde;

/* Interface & Structures */

pub trait ClipboardModel {
    fn new(opt: models::ModelHandlerOptions) -> Self;
    fn create_clipboard(&self, c: models::CreateClipboard) -> Result<String, Box<dyn Error>>;
    fn destroy_clipboard(&self, id: &str) -> Result<(), Box<dyn Error>>;
    fn retrieve_clipboard(&self);
}

pub struct ModelHandler {
    pub db: mongodb::db::Database,
    pub logger: slog::Logger,
    pub key: Vec<u8>,
}

pub struct ModelHandlerOptions {
    pub conn: mongodb::Client,
    pub key: String,
}

/* Implement */

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
    use mongodb::ThreadedClient;

    let _guard = slog_scope::set_global_logger(utils::new_logger(
        String::from("./log/") + "test.log",
        "test",
        false,
    ));
    let client = mongodb::Client::with_uri("mongodb://127.0.0.1:27017/").unwrap();
    models::ClipboardModel::new(models::ModelHandlerOptions {
        conn: client,
        key: String::from("test_salt"),
    })
}
