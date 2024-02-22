use crate::models::key::Key;
use crate::{DB_NAME, COLL_NAME, ALLOW_ADMIN};

use actix_web::{web, HttpResponse, post, HttpRequest};
use mongodb::{Client, Collection, bson::doc};

use serde_json::json;

#[post("/keys/remove")]
async fn remove_key(req: HttpRequest, client: web::Data<Client>, admin_key: web::Data<String>) -> HttpResponse {

    if *ALLOW_ADMIN == false {
        return HttpResponse::Locked().json(json!({"error": "Admin endpoints are disabled."}));
    }

    let req_key:&str  = req.headers().get("ADMIN_KEY")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if admin_key.to_string() != req_key {
        return HttpResponse::Unauthorized().json(json!({"error": "Invalid admin key."}));
    }

    let key_to_remove:&str  = req.headers().get("KEY_TO_REMOVE")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if key_to_remove.is_empty() {
        return HttpResponse::BadRequest().json(json!({"error": "The header 'KEY_TO_REMOVE' is missing or invalid."}));
    }

    let collection: Collection<Key> = client.database(&DB_NAME).collection(&COLL_NAME);
    let filter = doc! { "key": &key_to_remove };
    let delete_result = collection.delete_one(filter, None).await;

    match delete_result {
        Ok(delete_result) => {
            if delete_result.deleted_count > 0 {
                HttpResponse::Ok().json(json!({"message": "Key removed successfully."}))
            } else {
                HttpResponse::NotFound().json(json!({"error": "Key not found."}))
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}
