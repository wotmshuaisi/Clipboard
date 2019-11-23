use crate::models;
use std::error::Error;

pub struct Clipboard {
    id: String,
    clip_content: String,
    clip_type: i8,
    clip_onetime: bool,
    date_time: i64,
}

impl models::ClipboardModel for models::ModelHandler {
    fn create_clipboard() {}
    fn destroy_clipboard() {}
    fn retrieve_clipboard() {}
}
