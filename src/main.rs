extern crate actix_web;

use std::io::Result;

use rust_clean_code_api::run_server;

#[actix_web::main]
async fn main() -> Result<()> {
    run_server()?
    .await
}
