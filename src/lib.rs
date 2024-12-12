pub mod adapters;
pub mod domain;
pub mod infrastructure;
pub mod usecases;

use actix_web::dev::Server;

pub fn run_server() -> Result<Server, std::io::Error> {
    infrastructure::http_server::run()
}
