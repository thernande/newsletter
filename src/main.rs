use env_logger::Env;
use newsletter::run;
use std::net::TcpListener;

mod config;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = config::get_configuration().expect("Failed to read configuration.");

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
    let address = format!("{}:{}", &server_address, configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    println!("{:?} {:?}", server_address, port);
    run(listener)?.await
}
