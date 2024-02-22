use crate::{BUCKETS, PARENT_DIR, SERVE_AS_WEBP};
use actix_web::{get, web::Bytes, HttpRequest, HttpResponse, Result};
use image::ImageOutputFormat::WebP;
use serde_json::json;
use std::io::Cursor;
use std::path::PathBuf;

#[get("/{bucket}/{filename}")]
pub async fn serve(req: HttpRequest) -> Result<HttpResponse> {
    let extensions = ["jpg", "gif", "png", "webp"];

    let bucket: String = req
        .match_info()
        .get("bucket")
        .unwrap_or_default()
        .to_string();
    let filename: String = req
        .match_info()
        .get("filename")
        .unwrap_or_default()
        .to_string();

    let has_valid_extension = extensions
        .iter()
        .any(|ext| filename.to_lowercase().ends_with(ext));

    if !BUCKETS.contains(&bucket) {
        return Ok(HttpResponse::NotFound()
            .json(json!({"error": "The requested bucket does not exist or is not accessible."})));
    }

    let base_path = PathBuf::from(format!(
        "{}/{}/{}",
        PARENT_DIR.to_string(),
        bucket,
        filename
    ));

    let file_path = extensions
        .iter()
        .find_map(|ext| {
            let path = base_path.with_extension(ext);
            if path.exists() {
                Some(path)
            } else {
                None
            }
        })
        .unwrap_or(base_path);

    if !file_path.exists() || file_path.is_dir() {
        return Ok(HttpResponse::NotFound().json(json!({"error": "File not found."})));
    }

    if *SERVE_AS_WEBP {
        if has_valid_extension {
            match actix_files::NamedFile::open(file_path) {
                Ok(file) => Ok(file.into_response(&req)),
                Err(_) => Ok(HttpResponse::NotFound().json(json!({"error": "File not found."}))),
            }
        } else {
            match image::open(&file_path) {
                Ok(img) => {
                    let mut buffer = Cursor::new(Vec::new());
                    match img.write_to(&mut buffer, WebP) {
                        Ok(_) => {
                            let bytes = buffer.into_inner();
                            Ok(HttpResponse::Ok()
                                .content_type("image/webp")
                                .body(Bytes::from(bytes)))
                        }
                        Err(_) => Ok(HttpResponse::InternalServerError()
                            .json(json!({"error": "Failed to convert image to WebP."}))),
                    }
                }
                Err(_) => Ok(HttpResponse::InternalServerError()
                    .json(json!({"error": "Failed to open or recognize image file."}))),
            }
        }
    } else {
        match actix_files::NamedFile::open(file_path) {
            Ok(file) => Ok(file.into_response(&req)),
            Err(_) => Ok(HttpResponse::NotFound().json(json!({"error": "File not found."}))),
        }
    }
}
