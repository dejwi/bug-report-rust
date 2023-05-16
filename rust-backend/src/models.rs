use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct BugReportModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
