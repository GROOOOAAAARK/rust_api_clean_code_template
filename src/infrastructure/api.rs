extern crate actix_web;

use actix_web::web;

use crate::infrastructure::ping_apis;
use crate::infrastructure::certified_information_apis;

pub fn routes(service_config: &mut web::ServiceConfig) {
    service_config.service(web::scope("/ping").configure(ping_apis::routes));
    service_config.service(web::scope("/certified-information").configure(certified_information_apis::routes));
}
