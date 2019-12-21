use nanoid::generate;
use std::error::Error;
use std::fs::copy;

use crate::models;

impl models::StorageModel for models::ModelHandler {
    fn save_to_minio(
        &self,
        source: &str,
        folder: &str,
        file_name: &str,
    ) -> Result<String, Box<dyn Error>> {
        if source.is_empty() || file_name.is_empty() {
            return Ok(String::from(""));
        }
        let random_file_name = generate(3) + "_" + file_name;
        let url_file_path = format!("{}{}", folder, random_file_name);

        if let Err(err) = copy(
            source,
            String::from(&self.minio_public_path) + &url_file_path,
        ) {
            self.err_log("StorageModel save_to_minio", 1, &err.to_string());
            return Err(Box::new(err));
        };

        Ok(String::from(&self.minio_cdn_prefix) + &url_file_path)
    }
}
