use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson, ThreadedClient};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_derive::Serialize;

extern crate bcrypt;

use crate::models;
use crate::utils;

/* Constants */

const CLIPBOARD_COLLECTION_NAME: &str = "clipboard";
const ID_ALPHABETS: [char; 36] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/* Structure/Enums & methods */

#[derive(Debug)]
pub enum ClipboardType {
    Normal = 1,
    Markdown = 2,
    // Synatx,
}

impl ClipboardType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(ClipboardType::Normal),
            2 => Some(ClipboardType::Markdown),
            _ => None,
        }
    }
}

pub struct CreateClipboard {
    pub clip_content: String,
    pub clip_type: ClipboardType,
    pub clip_onetime: bool,
    pub is_lock: bool,
    pub password: Option<String>,
    pub expire_date: i64,
    // pub uid: Option<String>,
    // pub syntx_lang: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Clipboard {
    pub id: String,
    pub clip_type: u8,
    pub clip_content: String,
    pub clip_onetime: bool,
    pub expire_date: i64,
    pub date_time: i64,
    #[serde(skip_serializing)]
    pub is_lock: bool,
    #[serde(skip_serializing)]
    pub password: String,
}

impl Clipboard {
    pub fn is_expired(&self) -> bool {
        if self.expire_date
            <= SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
        {
            return true;
        }
        false
    }
    pub fn password_valid(&self, password: &str) -> bool {
        match bcrypt::verify(&self.password, password) {
            Ok(val) => val,
            Err(_) => false,
        }
    }
}

/* Models trait implement */

impl models::ClipboardModel for models::ModelHandler {
    fn new(opt: models::ModelHandlerOptions) -> Self {
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
        let cid = nanoid::custom(5, &ID_ALPHABETS);
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
                "expire_date": c.expire_date,
                "password": match c.password{
                    Some(val) => val,
                    None=> Default::default()
                },
                "date_time": SystemTime::now()
                .duration_since( UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
                "clip_type": c.clip_type as i32,
            },
            None,
        ) {
            Ok(val) => {
                if !val.inserted_id.is_some() {
                    self.warn_log("ClipboardModel create_clipboard", 1);
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
        match self.db.collection(CLIPBOARD_COLLECTION_NAME).delete_one(
            doc! {
                "id": id,
            },
            None,
        ) {
            Ok(val) => {
                if val.deleted_count == 0 {
                    self.warn_log("ClipboardModel destroy_clipboard", 0);
                }
                Ok(())
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    fn retrieve_clipboard(&self, id: &str) -> Result<Option<Clipboard>, Box<dyn Error>> {
        match self.db.collection(CLIPBOARD_COLLECTION_NAME).find_one(
            Some(doc! {
                "id": id,
            }),
            None,
        ) {
            Ok(val) => {
                if let Some(item) = val {
                    Ok(Some(Clipboard {
                        clip_type: item.get_i32("clip_type").unwrap() as u8,
                        id: String::from(String::from(
                            item.get_str("id").unwrap_or_else(|_| Default::default()),
                        )),
                        is_lock: item
                            .get_bool("is_lock")
                            .unwrap_or_else(|_| Default::default()),
                        date_time: item
                            .get_i64("date_time")
                            .unwrap_or_else(|_| Default::default()),
                        expire_date: item
                            .get_i64("expire_date")
                            .unwrap_or_else(|_| Default::default()),
                        clip_onetime: item
                            .get_bool("clip_onetime")
                            .unwrap_or_else(|_| Default::default()),
                        password: String::from(
                            item.get_str("password")
                                .unwrap_or_else(|_| Default::default()),
                        ),
                        clip_content: match item.get_binary_generic("clip_content") {
                            Ok(val) => {
                                let iv = item.get_str("iv");
                                if let Ok(iv) = iv {
                                    match utils::from_aes(&self.key, iv.as_bytes(), val) {
                                        Ok(val) => val,
                                        Err(err) => {
                                            self.err_log(
                                                "ClipboardModel retrieve_clipboard",
                                                1,
                                                &err.to_string(),
                                            );
                                            return Err(err);
                                        }
                                    }
                                } else {
                                    String::from("")
                                }
                            }
                            Err(_) => String::from(""),
                        },
                    }))
                } else {
                    Ok(None)
                }
            }
            Err(err) => {
                self.err_log("ClipboardModel retrieve_clipboard", 0, &err.to_string());
                Err(Box::new(err))
            }
        }
    }
}

/* Test functions */

#[test]
fn clipboard_test() {
    use crate::models::{ClipboardModel, CreateClipboard};

    let m = models::initial_test_handler();
    // let result = m
    //     .create_clipboard(CreateClipboard {
    //         clip_content: String::from("test"),
    //         clip_onetime: true,
    //         clip_type: ClipboardType::Normal,
    //         is_lock: false,
    //         password: None,
    //     })
    //     .unwrap();

    // assert_ne!(result, "");

    // // password content
    let pass = "password";
    let result = m
        .create_clipboard(CreateClipboard {
            clip_content: String::from("test 1"),
            clip_onetime: true,
            clip_type: ClipboardType::Normal,
            is_lock: true,
            password: Some(String::from(pass)),
            expire_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
                + (5 * 24 * 3600) as i64,
        })
        .unwrap();

    assert_ne!(result, "");

    // retrieve clipboard
    let doc = m.retrieve_clipboard(&result).unwrap();
    if let Some(doc) = doc {
        assert_ne!("", &doc.id);
        assert_eq!("test 1", &doc.clip_content);
        assert_eq!(true, doc.is_lock);
        assert_ne!(0, doc.date_time);
        assert_eq!(doc.clip_type as u8, 1);
    }

    // // delete clipboard
    let result = m.destroy_clipboard(&result);

    assert_eq!(result.is_ok(), true);
}
