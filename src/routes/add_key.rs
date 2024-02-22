use crate::models::key::Key;
use crate::utils::generate_key::generate_key;
use crate::{ALLOW_ADMIN, COLL_NAME, DB_NAME};

use actix_web::{post, web, HttpRequest, HttpResponse};
use mongodb::{Client, Collection};

use chrono::Utc;
use serde_json::json;

#[post("/keys/add")]
async fn add_key(
    req: HttpRequest,
    client: web::Data<Client>,
    admin_key: web::Data<String>,
) -> HttpResponse {

    if *ALLOW_ADMIN == false {
        return HttpResponse::Locked().json(json!({"error": "Admin endpoints are disabled."}));
    }

    let req_key: String = match req.headers().get("ADMIN_KEY") {
        Some(value) => match value.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => {
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Invalid admin key format."}));
            }
        },
        None => {
            return HttpResponse::BadRequest()
                .json(json!({"error": "The header 'ADMIN_KEY' is missing."}));
        }
    };

    if admin_key.to_string() != req_key {
        return HttpResponse::Unauthorized().json(json!({"error": "Invalid admin key."}));
    }

    let name: String = match req.headers().get("NAME") {
        Some(value) => match value.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => {
                return HttpResponse::BadRequest().json(json!({"error": "Invalid name format."}));
            }
        },
        None => {
            return HttpResponse::BadRequest()
                .json(json!({"error": "The header 'NAME' is missing."}));
        }
    };

    let gen_key = generate_key(client.clone()).await;

    let key: Key = Key {
        key: gen_key.clone(),
        username: name.clone(),
        added_at: Utc::now().to_string(),
    };

    let collection: Collection<Key> = client.database(&DB_NAME).collection(&COLL_NAME);
    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(key, None).await;
    match result {
        Ok(_) => HttpResponse::Ok()
            .json(json!({"message": "Key added", "key": gen_key, "username": name})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}
