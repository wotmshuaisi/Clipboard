mod model;
mod mongo_clipboard;

#[allow(unused_imports)]
use self::model::initial_test_handler;

pub use self::model::ClipboardModel;
pub use self::model::ModelHandler;
pub use self::model::ModelHandlerOptions;

pub use self::mongo_clipboard::Clipboard;
pub use self::mongo_clipboard::ClipboardType;
pub use self::mongo_clipboard::GetClipboard;
pub use self::mongo_clipboard::SetClipboard;
