use crate::config;
use crate::db;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};

pub mod route;
pub mod route_not_found;

pub async fn run(cfg: config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let port = cfg.http.port;
    println!("Server started at http://localhost:{}", port);
    let client = db::connect(&cfg.database).await;

    let client = Arc::new(Mutex::new(client));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(route::index)
            .default_service(web::route().to(route_not_found::route_not_found))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
    .map_err(Into::into)
}
