use actix_web::HttpResponse;

pub async fn route_not_found() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::NotFound().body("404 Not Found"))
}
