use std::str::FromStr;

use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    extractors::{AuthUser, Query},
    models::{BugReportModel, BugReportStatus},
    schema::{BugReportWithAuthorSchema, CreateBugReportSchema, UpdateBugReportSchema},
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
    .bind(user.id)
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
    let mut query = QueryBuilder::new("SELECT bugReports.id, title, description, status, bugReports.created_at, users.id as user_id, users.username as user_username FROM bugReports JOIN users on bugReports.author_id = users.id");

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

#[put("/bug-reports/{id}")]
pub async fn bug_report_update(
    id: web::Path<Uuid>,
    user: AuthUser,
    app_state: web::Data<AppState>,
    body: web::Json<UpdateBugReportSchema>,
) -> impl Responder {
    let report = match sqlx::query_as!(
        BugReportModel,
        r#"SELECT id, status as "status: _", author_id, title, description, created_at FROM bugReports WHERE id = $1"#,
        *id
    )
    .fetch_optional(&app_state.db)
    .await
    {
        Err(err) => {
            return HttpResponse::InternalServerError().json(json!({ "message": err.to_string() }))
        }
        Ok(None) => {
            return HttpResponse::NotFound().json(json!({ "message": "Bug report doesn't exist" }))
        }
        Ok(Some(report)) => report,
    };

    let is_author = user.id == report.author_id;
    if body.status == Some(BugReportStatus::SOLVED) && !is_author {
        return HttpResponse::Unauthorized().json(
            json!({"message": "You have to be an author in order to mark report as solved"}),
        );
    }

    match sqlx::query_as!(
        BugReportModel,
        r#"UPDATE bugReports SET title = $1, description = $2, status = $3 WHERE id = $4 RETURNING id, status as "status: _", author_id, title, description, created_at"#,
        body.title.as_ref().unwrap_or(&report.title),
        body.description.as_ref().or(report.description.as_ref()),
        body.status.as_ref().unwrap_or(&report.status) as _,
        *id
    )
    .fetch_one(&app_state.db)
    .await
    {
        Err(err) =>  HttpResponse::InternalServerError().json(json!({ "message": err.to_string() })),
        Ok(report) => HttpResponse::Ok().json(report)
    }
}

#[get("/bug-reports/{id}")]
pub async fn bug_report_one(app_state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match sqlx::query_as::<_, BugReportWithAuthorSchema>("SELECT bugReports.id, status, title, description, bugReports.created_at, users.id as user_id, users.username as user_username FROM bugReports JOIN users ON bugReports.author_id = users.id WHERE bugReports.id = $1")
    .bind(*id)
    .fetch_optional(&app_state.db)
    .await {
        Err(err) =>  HttpResponse::InternalServerError().json(json!({ "message": err.to_string() })),
        Ok(None) => HttpResponse::NotFound().json(json!({ "message": "Bug report not found" })),
        Ok(Some(report)) => HttpResponse::Ok().json(report)
    }
}
