use serde::Deserialize;

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
