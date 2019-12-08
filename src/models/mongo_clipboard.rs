use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson, ThreadedClient};
use std::error::Error;

extern crate bcrypt;
extern crate serde;

use crate::models;
use crate::utils;

const CLIPBOARD_COLLECTION_NAME: &str = "clipboard";
const ID_ALPHABETS: [char; 62] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

pub enum ClipboardType {
    Normal,
    Markdown,
    // Synatx,
}

pub struct CreateClipboard {
    pub clip_content: String,
    pub clip_type: ClipboardType,
    pub clip_onetime: bool,
    pub is_lock: bool,
    pub password: Option<String>,
    // pub uid: Option<String>,
    // pub syntx_lang: Option<String>,
}

impl models::ClipboardModel for models::ModelHandler {
    fn new(opt: models::ModelHandlerOptions) -> models::ModelHandler {
        use openssl::hash::{hash, MessageDigest};

        models::ModelHandler {
            db: opt.conn.db("clipboard"),
            logger: slog_scope::logger(),
            key: hash(MessageDigest::sha3_256(), opt.key.as_bytes())
                .unwrap()
                .to_vec(),
        }
    }

    fn create_clipboard(&self, mut c: CreateClipboard) -> Result<String, Box<dyn Error>> {
        let cid = nanoid::custom(12, &ID_ALPHABETS);
        let iv: String;
        let clip_content_encrypted: Vec<u8>;
        if c.is_lock {
            match bcrypt::hash(c.password.unwrap(), 4) {
                Ok(pass) => c.password = Some(pass),
                Err(err) => {
                    self.err_log("ClipboardModel create_clipboard", -1, &err.to_string());
                    return Err(Box::new(err));
                }
            }
        }
        match utils::to_aes(&self.key, c.clip_content.as_bytes()) {
            Ok(val) => {
                iv = val.0;
                clip_content_encrypted = val.1;
            }
            Err(err) => {
                self.err_log("ClipboardModel create_clipboard", 0, &err.to_string());
                return Err(err);
            }
        };

        match self.db.collection(CLIPBOARD_COLLECTION_NAME).insert_one(
            doc! {
                "id": String::from(cid.as_str()),
                "iv": iv,
                "clip_content": Bson::Binary(bson::spec::BinarySubtype::Generic, clip_content_encrypted),
                "clip_onetime": c.clip_onetime,
                "is_lock": c.is_lock,
                "password": match c.password{
                    Some(val) => val,
                    None=> Default::default()
                },
                "date_time":std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
                                            "clip_type": match c.clip_type {
                                                ClipboardType::Normal => 0,
                                                ClipboardType::Markdown => 1,
                                                // ClipboardType::Synatx => 2,
                                            },
            },
            None,
        ) {
            Ok(val) => {
                if !val.inserted_id.is_some() {
                    self.err_log("ClipboardModel create_clipboard", 1, "");
                }
                Ok(cid)
            }
            Err(err) => {
                self.err_log("ClipboardModel create_clipboard", 1, &err.to_string());
                Err(Box::new(err))
            }
        }
    }

    fn destroy_clipboard(&self, id: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn retrieve_clipboard(&self) {}
}

#[test]
fn create_clipboard_test() {
    use crate::models::{ClipboardModel, CreateClipboard};

    let m = models::initial_test_handler();
    let result = m
        .create_clipboard(CreateClipboard {
            clip_content: String::from("test"),
            clip_onetime: true,
            clip_type: ClipboardType::Normal,
            is_lock: false,
            password: None,
        })
        .unwrap();

    assert_ne!(result, "");
    // password content
    let pass = "password";
    let result = m
        .create_clipboard(CreateClipboard {
            clip_content: String::from("test 1"),
            clip_onetime: true,
            clip_type: ClipboardType::Normal,
            is_lock: true,
            password: Some(String::from(pass)),
        })
        .unwrap();

    assert_ne!(result, "");
}
