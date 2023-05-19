use std::path::Path;

use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    test, web, App, Error,
};
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, testing::TestSupport};

use bug_report_backend::{config::Config, services::set_services, AppState};

pub async fn get_test_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let args = sqlx::testing::TestArgs::new("/tests");
    let opts = sqlx::Postgres::test_context(&args)
        .await
        .unwrap()
        .connect_opts;

    let conn = PgPoolOptions::new().connect_with(opts).await.unwrap();
    let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
    migrator.run(&conn).await.unwrap();

    let config = Config::empty();
    let app_state = AppState { config, db: conn };

    test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .configure(set_services),
    )
    .await
}
