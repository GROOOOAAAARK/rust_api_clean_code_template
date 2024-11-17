extern crate actix_web;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io::Result;

use crate::infrastructure::api;

#[actix_web::main]
async fn main() -> Result<()> {
    const app = api::get_app();
    HttpServer::new(|| {
        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
