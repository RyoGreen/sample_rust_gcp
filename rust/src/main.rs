mod config;
mod db;
mod handler;

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let config = config::Config::from_file("config.json");
    if let Err(e) = handler::run(config).await {
        println!("Error: {}", e);
    };
    Ok(())
}
