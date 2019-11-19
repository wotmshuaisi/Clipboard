mod api_clipboard;
mod routers;

use self::api_clipboard::set_clipboard;
pub use self::routers::set_api_router;
