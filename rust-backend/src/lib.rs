use config::Config;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

pub mod config;
pub mod extractors;
pub mod models;
pub mod schema;
pub mod services;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub exp: i64,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub config: Config,
}

#[cfg(test)]
use actix_web::rt;
#[cfg(test)]
use sqlx::testing::TestSupport;

#[cfg(test)]
impl Drop for AppState {
    fn drop(&mut self) {
        rt::Runtime::new().unwrap().block_on(async {
            self.db.close().await;
            if let Err(error) = Postgres::cleanup_test_dbs().await {
                eprintln!("Couldn't clean test databases: {}", error.to_string());
            };
        })
    }
}
