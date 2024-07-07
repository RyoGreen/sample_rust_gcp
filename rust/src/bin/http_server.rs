use rust::config::config::Config;
use rust::handler::run;

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let config: Config = Config::from_file("config.json");
    if let Err(e) = run::run(config).await {
        println!("Error: {}", e);
    };
    Ok(())
}
