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
    pub frontend_url: String,
}

impl Config {
    /// Builds config from env values
    pub fn build() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }

    pub fn empty() -> Self {
        Self {
            postgres_host: "".to_string(),
            postgres_port: 1,
            postgres_password: "".to_string(),
            postgres_db: "".to_string(),
            database_url: "".to_string(),
            pgadmin_default_email: "".to_string(),
            pgadmin_default_password: "".to_string(),
            jwt_secret: "".to_string(),
            hash_secret: "".to_string(),
            frontend_url: "".to_string(),
        }
    }
}
