use actix_web::{get, HttpResponse, Responder};
#[get("/")]
async fn index() -> impl Responder {
    let file = std::fs::read_to_string("index.html").unwrap();
    HttpResponse::Ok().content_type("text/html").body(file)
}
