use actix_web::{post, web, HttpResponse, Responder};

use crate::{schema::AuthUserSchema, AppState};

#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    body: web::Json<AuthUserSchema>,
) -> impl Responder {
    if let Some(_) = sqlx::query!("SELECT * FROM users WHERE username = $1", body.username)
        .fetch_optional(&app_state.db)
        .await
        .unwrap()
    {
        return HttpResponse::Conflict();
    }

    HttpResponse::Ok()
}
