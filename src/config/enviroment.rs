use dotenv::dotenv;
use std::env;

pub struct Enviroment {
    pub server_address: String,
    pub port: u16,
}

impl Enviroment {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();
        Ok(Self {
            server_address: env::var("SERVER_ADDRESS")?,
            port: env::var("PORT")?.parse::<u16>().unwrap(),
        })
    }
}
