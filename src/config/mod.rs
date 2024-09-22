use dotenv::dotenv;
use std::env;

pub struct Config {
    pub port: u16,
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            port: env::var("APP_PORT")
                .expect("APP_PORT must be set")
                .parse()
                .unwrap(),
            db_url: env::var("DATABASE_URL").expect("DB_URL nmust be set"),
        }
    }
}
