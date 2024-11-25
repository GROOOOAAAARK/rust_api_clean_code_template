extern crate actix_web;

use actix_web::web;

use crate::infrastructure::ping_apis;

pub fn routes(service_config: &mut web::ServiceConfig) {
    service_config.service(web::scope("/ping").configure(ping_apis::routes));
}
