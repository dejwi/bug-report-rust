use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo},
    FromRow, Type,
};
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Type, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "bugReportStatus")]
pub enum BugReportStatus {
    OPEN,
    CLOSED,
    SOLVED,
    REVIEW,
    ACCEPTED,
}

impl PgHasArrayType for BugReportStatus {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        PgTypeInfo::with_name("_bugReportStatus")
    }
}

#[derive(Serialize, FromRow, Deserialize)]
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
