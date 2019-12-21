mod api_clipboard;
mod api_storage;
mod routers;

use self::api_clipboard::create_clipboard;
use self::api_clipboard::islock_clipboard;
use self::api_clipboard::retrieve_clipboard;
use self::api_clipboard::set_clipboard;
use self::api_storage::upload_to_storage;

pub use self::routers::set_api_router;
pub use self::routers::HandlerState;
