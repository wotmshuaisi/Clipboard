mod model;
mod mongo_clipboard;

#[allow(unused_imports)]
use self::model::initial_test_handler;

pub use self::model::ClipboardModel;
pub use self::model::ModelHandler;
pub use self::mongo_clipboard::Clipboard;
