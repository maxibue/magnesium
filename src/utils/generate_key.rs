use rand::{ distributions::Alphanumeric, Rng };
use std::time::{ SystemTime, UNIX_EPOCH };
use actix_web::web;
use mongodb::{ Client, Collection };
use crate::models::key::Key;
use crate::{ DB_NAME, COLL_NAME };
use mongodb::bson::doc;

async fn existance_check(client: web::Data<Client>, key: String) -> bool {
    let collection: Collection<Key> = client.database(&DB_NAME).collection(&COLL_NAME);
    match collection.find_one(doc! { "key": &key }, None).await {
        Ok(Some(_user)) => true,
        Ok(None) => false,
        Err(_err) => true,
    }
}

pub async fn generate_key(client: web::Data<Client>) -> String {
    let start_epoch = 1609459200;
    loop {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32;
        let time_component = current_time - start_epoch;

        let rng = rand::thread_rng();
        let random_string: String = rng
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();

        let api_key = format!("key_{}_{}", time_component, random_string);

        if !existance_check(client.clone(), api_key.clone()).await {
            return api_key;
        }
    }
}
