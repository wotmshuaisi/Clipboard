use mongodb::db::ThreadedDatabase;
use mongodb::{to_bson, Bson, ThreadedClient};

use crate::models;

extern crate serde;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Clipboard {
    pub id: String,
    pub clip_content: String,
    pub clip_type: i8,
    pub clip_onetime: bool,
    pub date_time: i64,
    // pub uid: String,
}

impl models::ClipboardModel for models::ModelHandler {
    fn new(dbcon: mongodb::Client) -> models::ModelHandler {
        models::ModelHandler {
            db: dbcon.db("clipboard"),
            logger: slog_scope::logger(),
        }
    }

    fn create_clipboard(&self, mut c: Clipboard) -> bool {
        c.date_time = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        match to_bson(&c) {
            Ok(val) => match val {
                Bson::Document(val) => {
                    match self.db.collection("clipboard").insert_one(val, None) {
                        Ok(val) =val.acknowledged,
                        Err(err) => {
                            error!(
                                self.logger,
                                "{}{}",
                                err,
                                self.err_location("ClipboardModel", "create_clipboard", 2)
                            );
                            false
                        }
                    }
                }
                _ => {
                    error!(
                        self.logger,
                        "Unable to convert bson to Document {}",
                        self.err_location("ClipboardModel", "create_clipboard", 1)
                    );
                    false
                }
            },
            Err(err) => {
                error!(
                    self.logger,
                    "{}{}",
                    err,
                    self.err_location("ClipboardModel", "create_clipboard", 0)
                );
                false
            }
        }
    }

    fn destroy_clipboard(&self) {}

    fn retrieve_clipboard(&self) {}
}

#[test]
fn create_clipboard_test() {
    use crate::models::{Clipboard, ClipboardModel};

    let m = models::initial_test_handler();

    assert_eq!(
        m.create_clipboard(Clipboard {
            id: String::from("test_id"),
            clip_content: String::from("test_content"),
            clip_type: 0,
            clip_onetime: true,
            date_time: 0,
        }),
        true
    )
}
