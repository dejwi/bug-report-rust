use std::str::FromStr;

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    extractors::{AuthUser, Query},
    models::BugReportStatus,
    schema::{BugReportWithAuthorSchema, CreateBugReportSchema},
    AppState,
};

#[post("/bug-reports")]
pub async fn bug_report_create(
    user: AuthUser,
    body: web::Json<CreateBugReportSchema>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    match sqlx::query_as::<_, BugReportWithAuthorSchema>(
        "WITH inserted_report as (INSERT INTO bugReports (title, description, author_id) VALUES ($1, $2, $3) RETURNING *) 
        SELECT inserted_report.id, status, title, inserted_report.created_at, description, users.id as user_id, users.username as user_username 
        FROM inserted_report JOIN users ON inserted_report.author_id = users.id",
    )
    .bind(&body.title)
    .bind(&body.description)
    .bind(Uuid::from_str(&user.id[..]).unwrap())
    .fetch_one(&app_state.db)
    .await
    {
        Err(err) => HttpResponse::InternalServerError().json(json!({ "message": err.to_string() })),
        Ok(bug_report) => HttpResponse::Ok().json(bug_report),
    }
}

#[derive(Deserialize, Debug)]
pub struct FindAllQuery {
    pub status: Option<Vec<BugReportStatus>>,
}

#[get("/bug-reports")]
pub async fn bug_report_all(
    app_state: web::Data<AppState>,
    filters: Query<FindAllQuery>,
) -> impl Responder {
    let mut query = QueryBuilder::new("SELECT *, users.id as user_id, users.username as user_username FROM bugReports JOIN users on bugReports.author_id = users.id");

    if let Some(ref statuses) = filters.status {
        query.push(" AND status = ANY(");
        query.push_bind(statuses);
        query.push(") ");
    }

    match query
        .build_query_as::<BugReportWithAuthorSchema>()
        .fetch_all(&app_state.db)
        .await
    {
        Err(err) => HttpResponse::InternalServerError().json(json!({ "message": err.to_string() })),
        Ok(bug_reports) => HttpResponse::Ok().json(bug_reports),
    }
}
