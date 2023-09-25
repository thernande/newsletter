use crate::routes;
use actix_files as fs;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
//use tracing_actix_web::TracingLogger;

// #[tokio::main])
pub fn run(port: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // .service(fs::Files::new("/static", ".").show_files_listing()) //ruta para validar el rutero principal
            // se crea un grupo de de enlaces para mejor organizacion y formacion de la api
            .service(web::scope("/api").configure(routes::api))
            .service(
                fs::Files::new("/", "./public")
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            .app_data(db_pool.clone())
            .wrap(Logger::default())
    })
    .listen(port)?
    .run();
    Ok(server)
}
