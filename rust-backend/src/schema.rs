use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use uuid::Uuid;

use crate::models::BugReportStatus;

#[derive(Deserialize)]
pub struct AuthUserSchema {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateBugReportSchema {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct BasicUserSchema {
    #[sqlx(rename = "user_id")]
    pub id: Uuid,
    #[sqlx(rename = "user_username")]
    pub username: String,
}

#[derive(Serialize, FromRow)]
pub struct BugReportWithAuthorSchema {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: BugReportStatus,
    #[sqlx(flatten)]
    pub author: BasicUserSchema,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
