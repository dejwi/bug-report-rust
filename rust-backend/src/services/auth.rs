use actix_web::{get, post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use crate::{extractors::AuthUser, models::UserModel, schema::AuthUserSchema, AppState, Claims};

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
            return HttpResponse::InternalServerError().json(json!({ "message": err.to_string() }));
        }
    }
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    body: web::Json<AuthUserSchema>,
) -> impl Responder {
    let user = match sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE username = $1",
        body.username
    )
    .fetch_optional(&app_state.db)
    .await
    .unwrap()
    {
        None => {
            return HttpResponse::NotFound()
                .json(json!({"message": "Account with that username doesn't exist"}))
        }
        Some(user) => user,
    };

    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");
    let argon2 = Argon2::new_with_secret(
        hash_secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    )
    .unwrap();

    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    match argon2.verify_password(body.password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

            let exp = (Utc::now() + Duration::days(30)).timestamp();
            let claims = Claims {
                id: user.id.to_string(),
                exp,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            )
            .unwrap();

            HttpResponse::Ok().json(json!({ "token": token }))
        }
        Err(_) => HttpResponse::Unauthorized().json(json!({"message": "Invalid password"})),
    }
}

#[get("/me")]
pub async fn account_information(user: AuthUser, app_state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", user.id)
        .fetch_one(&app_state.db)
        .await
    {
        Err(err) => HttpResponse::InternalServerError().json(json!({ "message": err.to_string() })),
        Ok(user) => HttpResponse::Ok().json(user),
    }
}
