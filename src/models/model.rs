extern crate serde;

use crate::models;

pub trait ClipboardModel {
    fn new(dbcon: mongodb::Client) -> Self;
    fn create_clipboard(&self, c: models::Clipboard) -> bool;
    fn destroy_clipboard(&self);
    fn retrieve_clipboard(&self);
}

pub struct ModelHandler {
    pub db: mongodb::db::Database,
    pub logger: slog::Logger,
}

impl ModelHandler {
    pub fn err_location(&self, model_name: &str, function_name: &str, index: i8) -> String {
        format!(" [MD:{} FN:{} I:{}]", model_name, function_name, index)
    }
}

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
    models::ClipboardModel::new(client)
}
