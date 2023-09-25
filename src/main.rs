// use env_logger::Env;
use newsletter::startup::run;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

mod config;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = config::get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let connection = PgPool::connect(&connection_string.expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let mut server_address: String = "0.0.0.0".to_string();
    let mut port: u16 = 3000;
    //se toma el servidor y puerto por el cual funcionara la app desde las variables de entorno
    match config::enviroment::Enviroment::from_env() {
        Ok(env_vars) => {
            server_address = env_vars.server_address;
            port = env_vars.port;
        }
        Err(e) => println!("couldn't get env vars: {}", e),
    }
    let address = format!("{}:{}", &server_address, configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    println!("{:?} {:?}", server_address, port);
    run(listener, connection)?.await
}
