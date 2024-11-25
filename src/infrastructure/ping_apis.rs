extern crate actix_web;

use actix_web::{get, HttpResponse, web::ServiceConfig};

#[get("")]
async fn pong() -> Result<HttpResponse, std::io::Error> {
    Ok(HttpResponse::Ok().body("pong"))
}

pub fn routes(service_config: &mut ServiceConfig) {
    service_config.service(pong);
}
