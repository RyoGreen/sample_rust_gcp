use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let port = 8080;
    println!("Server started at http://localhost:{}", port);
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .default_service(web::route().to(route_not_found))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;
    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    let file = std::fs::read_to_string("index.html").unwrap();
    HttpResponse::Ok().content_type("text/html").body(file)
}

async fn route_not_found() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::NotFound().body("404 Not Found"))
}
