use actix_web::web;
use mongodb::{Client, Collection};
use crate::models::key::Key;
use crate::{DB_NAME, COLL_NAME};
use mongodb::bson::doc;

pub async fn check_key(client: web::Data<Client>, key: String) -> bool {
    let collection: Collection<Key> = client.database(&DB_NAME).collection(&COLL_NAME);
    match collection
        .find_one(doc! { "key": &key }, None)
        .await
    {
        Ok(Some(_user)) => true,
        Ok(None) => false,
        Err(_err) => false,
    }
}