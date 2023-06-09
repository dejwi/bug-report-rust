use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use uuid::Uuid;

use crate::models::BugReportStatus;

#[derive(Deserialize, Serialize)]
pub struct AuthUserSchema {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, FromRow, Deserialize)]
pub struct BasicUserSchema {
    #[sqlx(rename = "user_id")]
    pub id: Uuid,
    #[sqlx(rename = "user_username")]
    pub username: String,
}

#[derive(Serialize, FromRow, Deserialize)]
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

#[derive(Deserialize, Serialize)]
pub struct CreateBugReportSchema {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateBugReportSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<BugReportStatus>,
}
