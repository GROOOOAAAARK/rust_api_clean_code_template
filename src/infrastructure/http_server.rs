use crate::infrastructure::api;

use actix_web::{App, HttpServer, dev::Server};

pub fn server() -> Result<Server, std::io::Error> {
    const port: u16 = 8080;//TODO: move to env var

    let server: Server = HttpServer::new(move || App::new()
        .configure(api::routes)
    )
    .bind(("127.0.0.1", port)).expect("Failed to bind to port")// TODO: use vars and env, maybe use tcp listener
    .run();

    println!("✨ Server running on port {} 🚀", port);

    Ok(server)
}
