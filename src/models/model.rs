pub trait ClipboardModel {
    fn create_clipboard();
    fn destroy_clipboard();
    fn retrieve_clipboard();
}

pub struct ModelHandler {}

pub fn new() -> ModelHandler {
    return ModelHandler {};
}
