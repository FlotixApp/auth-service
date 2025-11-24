mod config;
mod google;

use actix_web::{App, HttpServer, web};
use crate::config::Config;
use crate::google::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = Config::load();

    println!("Starting server on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cfg.clone()))
            .service(google_login)
            .service(google_callback)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

