use crate::{ALLOW_RESIZING, BUCKETS, PARENT_DIR};
use crate::utils::check_key::check_key;

use actix_web::{post, web, HttpRequest, HttpResponse};
use std::path::{Path, PathBuf};

use actix_multipart::Multipart;
use futures::TryStreamExt;
use image;
use mongodb::Client;
use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

#[post("/upload/{bucket}")]
pub async fn upload(
    req: HttpRequest,
    client: web::Data<Client>,
    bucket: web::Path<String>,
    mut payload: Multipart,
) -> HttpResponse {
    let api_key = req
        .headers()
        .get("API_KEY")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    if !check_key(client, api_key.to_string()).await {
        return HttpResponse::Unauthorized()
            .json(json!({"error": "Invalid or unauthorized API key."}));
    }

    let bucket_str = bucket.into_inner();

    if !BUCKETS.contains(&bucket_str) {
        return HttpResponse::NotFound()
            .json(json!({"error": "The requested bucket does not exist or is not accessible."}));
    }

    let data_path: String = format!("./{}/{}", PARENT_DIR.to_string(), bucket_str); //PARENT_DIR.to_string()
    if !Path::new(&data_path).exists() {
        return HttpResponse::NotFound()
            .json(json!({"error": "The requested bucket does not exist."}));
    }

    let mut file_info = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap_or("unnamed");
        let file_extension = Path::new(&filename)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        let uuid_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
        let file_path = PathBuf::from(&data_path).join(&uuid_filename);

        let mut f = match File::create(&file_path) {
            Ok(file) => file,
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"error": "Failed to create file."}))
            }
        };

        let mut total_size: usize = 0;
        const MAX_SIZE: usize = 1 * 1024 * 1024 * 1024; // 1GB in bytes

        while let Ok(Some(chunk)) = field.try_next().await {
            total_size += chunk.len();
            if total_size > MAX_SIZE {
                // Remove the partially written file
                let _ = std::fs::remove_file(&file_path);
                return HttpResponse::PayloadTooLarge()
                    .json(json!({"error": "File size exceeds the 1GB limit."}));
            }

            if let Err(_) = f.write_all(&chunk) {
                return HttpResponse::InternalServerError()
                    .json(json!({"error": "Error writing file."}));
            }
        }

        // Extract width and height from request headers (if available)
        let width: Option<u32> = req
            .headers()
            .get("width")
            .and_then(|hv| hv.to_str().ok()?.parse().ok());
        let height: Option<u32> = req
            .headers()
            .get("height")
            .and_then(|hv| hv.to_str().ok()?.parse().ok());
        // Pass width and height to the process_file function
        match process_file(*ALLOW_RESIZING, &file_path, width, height) {
            Ok(_) => file_info.push((uuid_filename.clone(), bucket_str.clone())),
            Err(e) => {
                let _ = fs::remove_file(&file_path);
                return HttpResponse::BadRequest().json(json!({"error": e}));
            }
        }
    }

    if !file_info.is_empty() {
        HttpResponse::Ok().json(
            json!({"message": "File(s) uploaded and processed successfully.", "files": file_info}),
        )
    } else {
        HttpResponse::BadRequest().json(json!({"error": "No files were uploaded."}))
    }
}

fn process_file(
    allow_resizing: bool,
    file_path: &PathBuf,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<(), &'static str> {
    let mut img: image::DynamicImage = match image::open(file_path) {
        Ok(img) => img,
        Err(_) => return Err("Failed to open or recognize image file."),
    };

    if allow_resizing {
        if let (Some(w), Some(h)) = (width, height) {
            img = img.resize_exact(w, h, image::imageops::FilterType::Nearest);
        }
    }

    match img.save(file_path) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to save processed image."),
    }
}
