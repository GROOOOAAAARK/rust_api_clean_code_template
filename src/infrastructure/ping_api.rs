extern crate actix_web;

use actix_web::{AppService, get, HttpRequest, HttpResponse, Responder, web}

#[get("/pong")]
fn pong() -> impl Responder{
    HttpResponse::Ok().body("pong");
}

fn get_routes() -> Vec<fn> {
    let route_vec: Vec<fn> = Vec::new();
    route_vec.push(pong);
    route_vec
}

pub fn get_router() -> AppService {
    let router: web::scope("/ping");
    for route in get_routes() {
        router = router.service(route);
    }
    router
}