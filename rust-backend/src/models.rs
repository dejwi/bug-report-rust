use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::Type, Serialize)]
pub enum BugReportStatus {
    OPEN,
    CLOSED,
    SOLVED,
    REVIEW,
    ACCEPTED,
}

#[derive(Serialize, FromRow)]
pub struct BugReportModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: BugReportStatus,
    #[serde(rename = "authorId")]
    pub author_id: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
