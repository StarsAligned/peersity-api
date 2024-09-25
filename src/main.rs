mod config;
mod health;

use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};
use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _config: Config = Config::new().map_err(|e| {
        eprintln!("Failed to load configuration: {}", e);
        return std::io::Error::new(std::io::ErrorKind::Other, "Configuration error");
    })?;

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://peersity.com")
            .allowed_origin("http://alpha.peersity.com")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .configure(health::endpoints::configure_endpoints)
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    println!("Server started at http://localhost:8080");

    return server.await;
}
