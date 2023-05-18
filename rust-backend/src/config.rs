use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    postgres_host: String,
    postgres_port: u16,
    postgres_password: String,
    postgres_db: String,
    pub database_url: String,
    pgadmin_default_email: String,
    pgadmin_default_password: String,
    pub jwt_secret: String,
    pub hash_secret: String,
}

impl Config {
    /// Builds config from env values
    pub fn build() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }
}
