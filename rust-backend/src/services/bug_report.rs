use std::str::FromStr;

use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

use crate::{
    extractors::AuthUser, models::BugReportModel, schema::CreateBugReportSchema, AppState,
};

#[post("/bug-report")]
pub async fn bug_report_create(
    user: AuthUser,
    body: web::Json<CreateBugReportSchema>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    match sqlx::query_as!(
        BugReportModel,
        r#"INSERT INTO bugReports (title, description, author_id) VALUES ($1, $2, $3) RETURNING id, title, description, author_id, status as "status: _", created_at"#,
        body.title,
        body.description,
        Uuid::from_str(&user.id[..]).unwrap()
    )
    .fetch_one(&app_state.db)
    .await
    {
        Err(err) => HttpResponse::InternalServerError().json(json!({ "message": err.to_string() })),
        Ok(bug_report) => HttpResponse::Ok().json(bug_report),
    }
}
