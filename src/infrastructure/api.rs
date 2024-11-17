extern crate actix_web;

use actix_web::{App, HttpServer}

use crate::infrastructure::ping_api;

fn module_routes() -> Vec<fn> {
    const routes: Vec<fn> = Vec::new();
    routes.extend(ping_api::get_router());
}

pub fn get_app() -> App {
    const app = App::new();
    for route in module_routes() {
        app.service(route);
    }
    app
}

