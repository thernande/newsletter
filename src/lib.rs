use actix_files as fs;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

// use std::fs::read_dir;

pub mod config;
mod controller;
mod model;
mod routes;
// #[tokio::main])
pub fn run(port: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // .service(fs::Files::new("/static", ".").show_files_listing()) //ruta para validar el rutero principal
            // se crea un grupo de de enlaces para mejor organizacion y formacion de la api
            .service(web::scope("/api").configure(routes::api))
            .service(
                fs::Files::new("/", "./public")
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            .wrap(Logger::default())
    })
    .listen(port)?
    .run();
    Ok(server)
}
