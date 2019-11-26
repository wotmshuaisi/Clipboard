#[macro_use]
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson};
use mongodb::{Client, ThreadedClient};

pub trait ClipboardModel {
    fn create_clipboard();
    fn destroy_clipboard();
    fn retrieve_clipboard();
}

pub struct ModelHandler {
    db: mongodb::db::Database,
}

pub fn new(dbcon: mongodb::Client) -> ModelHandler {
    return ModelHandler {
        db: dbcon.db("clipboard"),
    };
}
