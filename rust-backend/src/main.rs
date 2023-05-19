use actix_web::{middleware::Logger, web, App, HttpServer};
use bug_report_backend::{config::Config, services, AppState};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, testing::TestSupport, Postgres};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = match Config::build() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Failed to build config from env: {}", error.to_string());
            std::process::exit(1);
        }
    };

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info,error,warn");
    }
    env_logger::init();

    if let Err(error) = Postgres::cleanup_test_dbs().await {
        log::error!("Couldn't clean test databases: {}", error.to_string());
    }

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Failed to connect with the database {:?}", err);
            std::process::exit(1);
        }
    };

    let app_state = AppState { db: pool, config };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .configure(services::set_services)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
