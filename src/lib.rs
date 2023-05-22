use actix_files as fs;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
// use std::fs::read_dir;

mod config;
mod controller;
mod routes;

// #[tokio::main]
pub fn run() -> Result<Server, std::io::Error> {
    let mut server_address: String = "0.0.0.0".to_string();
    let mut port: u16 = 3000;
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    //se toma el servidor y puerto por el cual funcionara la app desde las variables de entorno
    match config::enviroment::Enviroment::from_env() {
        Ok(env_vars) => {
            server_address = env_vars.server_address;
            port = env_vars.port;
        }
        Err(e) => println!("couldn't get env vars: {}", e),
    }
    println!("{:?} {:?}", &server_address, &port);
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
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
    .bind((server_address, port))?
    .run();
    Ok(server)
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}
