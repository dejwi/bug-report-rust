use actix_web::{
    delete, get, post, put,
    web::{self, Json},
};
use serde::Deserialize;
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    error::{self, ServerError},
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
) -> error::Result<BugReportWithAuthorSchema> {
    let report = sqlx::query_as::<_, BugReportWithAuthorSchema>(
        "WITH inserted_report as (INSERT INTO bugReports (title, description, author_id) VALUES ($1, $2, $3) RETURNING *) 
        SELECT inserted_report.id, status, title, inserted_report.created_at, description, users.id as user_id, users.username as user_username 
        FROM inserted_report JOIN users ON inserted_report.author_id = users.id",
    )
    .bind(&body.title)
    .bind(&body.description)
    .bind(user.id)
    .fetch_one(&app_state.db)
    .await?;

    Ok(Json(report))
}

#[derive(Deserialize, Debug)]
pub struct FindAllQuery {
    pub status: Option<Vec<BugReportStatus>>,
}

#[get("/bug-reports")]
pub async fn bug_report_all(
    app_state: web::Data<AppState>,
    filters: Query<FindAllQuery>,
) -> error::Result<Vec<BugReportWithAuthorSchema>> {
    let mut query = QueryBuilder::new("SELECT bugReports.id, title, description, status, bugReports.created_at, users.id as user_id, users.username as user_username FROM bugReports JOIN users on bugReports.author_id = users.id");

    if let Some(ref statuses) = filters.status {
        query.push(" AND status = ANY(");
        query.push_bind(statuses);
        query.push(") ");
    }

    query.push(" ORDER BY bugReports.created_at desc");

    let bug_reports = query
        .build_query_as::<BugReportWithAuthorSchema>()
        .fetch_all(&app_state.db)
        .await?;

    Ok(Json(bug_reports))
}

#[put("/bug-reports/{id}")]
pub async fn bug_report_update(
    id: web::Path<Uuid>,
    user: AuthUser,
    app_state: web::Data<AppState>,
    body: web::Json<UpdateBugReportSchema>,
) -> error::Result<BugReportModel> {
    let report = match sqlx::query_as!(
        BugReportModel,
        r#"SELECT id, status as "status: _", author_id, title, description, created_at FROM bugReports WHERE id = $1"#,
        *id
    )
    .fetch_optional(&app_state.db)
    .await?
    {
        None => {
            return Err(ServerError::NotFound("Bug report doesn't exist"));
        }
       Some(report) => report,
    };

    let is_author = user.id == report.author_id;

    if (body.description.is_some() || body.title.is_some()) && !is_author {
        return Err(ServerError::Unauthorized(
            "You have to be an author in order to edit report title, description",
        ));
    }

    if body.status == Some(BugReportStatus::SOLVED) && !is_author {
        return Err(ServerError::NotFound(
            "You have to be an author in order to mark report as solved",
        ));
    }

    let report =  sqlx::query_as!(
        BugReportModel,
        r#"UPDATE bugReports SET title = $1, description = $2, status = $3 WHERE id = $4 RETURNING id, status as "status: _", author_id, title, description, created_at"#,
        body.title.as_ref().unwrap_or(&report.title),
        body.description.as_ref().or(report.description.as_ref()),
        body.status.as_ref().unwrap_or(&report.status) as _,
        *id
    )
    .fetch_one(&app_state.db)
    .await?;

    Ok(Json(report))
}

#[get("/bug-reports/{id}")]
pub async fn bug_report_one(
    app_state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> error::Result<BugReportWithAuthorSchema> {
    match sqlx::query_as::<_, BugReportWithAuthorSchema>("SELECT bugReports.id, status, title, description, bugReports.created_at, users.id as user_id, users.username as user_username FROM bugReports JOIN users ON bugReports.author_id = users.id WHERE bugReports.id = $1")
    .bind(*id)
    .fetch_optional(&app_state.db)
    .await? {
        None => Err(ServerError::NotFound("Bug report not found")),
        Some(report) => Ok(Json(report))
    }
}

#[delete("/bug-reports/{id}")]
pub async fn bug_report_delete(
    id: web::Path<Uuid>,
    user: AuthUser,
    app_state: web::Data<AppState>,
) -> error::Result<&'static str> {
    let report = match sqlx::query_as!(
        BugReportModel,
        r#"SELECT id, status as "status: _", author_id, title, description, created_at FROM bugReports WHERE id = $1"#,
        *id
    )
    .fetch_optional(&app_state.db)
    .await?
    {
        None => {
            return Err(ServerError::NotFound("Bug report doesn't exist"))
        }
        Some(report) => report,
    };

    if user.id != report.author_id {
        return Err(ServerError::Unauthorized(
            "Only author can delete his own bug report",
        ));
    }

    sqlx::query!("DELETE FROM bugReports WHERE id = $1 RETURNING id", *id)
        .fetch_one(&app_state.db)
        .await?;

    Ok(Json(""))
}
