pub fn get_env(key: &str, default: &str) -> String {
    match std::env::var("MODE") {
        Ok(val) => String::from(val),
        Err(_) => String::from(default),
    }
}
