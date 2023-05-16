use actix_web::{post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, Version,
};
use serde_json::json;

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
        return HttpResponse::Conflict()
            .json(json!({"message": "Account with that username already exists"}));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");

    let argon2 = Argon2::new_with_secret(
        hash_secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    )
    .unwrap();

    let password_hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    match sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id",
        body.username,
        password_hash
    )
    .fetch_one(&app_state.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Account created successfully"})),
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(json!({ "message": format!("{:?}", err) }));
        }
    }
}
