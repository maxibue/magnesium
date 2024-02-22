// #[cfg(test)]
// mod test;
mod routes;
mod utils;
mod models;

use actix_web::{web, App, HttpServer};

use dotenv::dotenv;
use mongodb::Client;
use once_cell::sync::Lazy;
use actix_cors::Cors;


use utils::config::CONFIG;
use utils::buckets_setup::buckets_setup;
use utils::print_magnesium::print_magnesium;

pub static DB_NAME: Lazy<String> = Lazy::new(|| CONFIG.db_name.clone());
pub static COLL_NAME: Lazy<String> = Lazy::new(|| CONFIG.collection_name.clone());
pub static BUCKETS: Lazy<Vec<String>> = Lazy::new(|| CONFIG.buckets.clone());
pub static PARENT_DIR: Lazy<String> = Lazy::new(|| CONFIG.parent_directory.clone());
pub static ALLOW_ADMIN: Lazy<bool> = Lazy::new(|| CONFIG.allow_admin.clone());
pub static SERVE_AS_WEBP: Lazy<bool> = Lazy::new(|| CONFIG.serve_as_webp.clone());
pub static ALLOW_RESIZING: Lazy<bool> = Lazy::new(|| CONFIG.allow_resizing.clone());

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print_magnesium();
    dotenv().ok();

    buckets_setup(CONFIG.parent_directory.clone(), CONFIG.buckets.clone());

    let admin_key = match std::env::var("ADMIN_KEY") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error reading \"ADMIN_KEY\" environmental variable: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        },
    };

    let uri = match std::env::var("MONGO_URI") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error reading \"MONGO_URI\" environmental variable: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        },
    };

    let client = match Client::with_uri_str(&uri).await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to connect to MongoDB: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        },
    };

    println!("\nConnection to MongoDB established.");
    
    let service_ip = match std::env::var("SERVICE_IP") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error reading \"SERVICE_IP\" environmental variable: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        },
    };

    let port = match std::env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(e) => {
            eprintln!("Error reading \"PORT\" environmental variable: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        },
    };

    if *ALLOW_ADMIN == true {
        println!("\nAdmin endpoints are enabled & exposed!");
    }

    println!("\nService running at: http://{}/", &service_ip);
    println!("With port: {}", &port);

    HttpServer::new(move || {

        let cors = Cors::default()
            .allowed_origin("http://example.com")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
        .wrap(cors)
        .app_data(web::Data::new(client.clone()))
        .app_data(web::Data::new(admin_key.clone()))
        .service(routes::upload::upload)
        .service(routes::serve::serve)
        .service(routes::add_key::add_key)
        .service(routes::remove_key::remove_key)
    })
    .bind((service_ip, port))?
    .run()
    .await
}