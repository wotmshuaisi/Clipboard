use crate::utils;
use actix_multipart::Multipart;
use actix_web::web;
use futures::StreamExt;
use std::collections::HashMap;
use std::io::Write;

pub async fn multipart_processor(
    mut tmp_path: String,
    file_field: &str,
    mut payload: Multipart,
) -> Result<HashMap<String, String>, actix_multipart::MultipartError> {
    let mut result: HashMap<String, String> = HashMap::new();
    let mut files = String::from("");

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition().unwrap();
        let field_name = content_disposition.get_name().unwrap();

        if field_name != file_field {
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                result.insert(
                    String::from(field_name),
                    match String::from_utf8(data.to_vec()) {
                        Ok(val) => val,
                        Err(_) => String::from(""),
                    },
                );
            }
            continue;
        }
        let file_name = content_disposition.get_filename().unwrap();
        let path = tmp_path.clone() + &nanoid::simple();
        files = files + file_name + "|" + &path + "||";

        let mut f = web::block(|| std::fs::File::create(path)).await.unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap();
        }
    }

    if !files.is_empty() {
        result.insert(String::from(file_field), files);
    }

    Ok(result)
}
