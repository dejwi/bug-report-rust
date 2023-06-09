use actix_web::{
    get, post,
    web::{self, Json},
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    error::{self, ServerError},
    extractors::AuthUser,
    models::UserModel,
    schema::AuthUserSchema,
    AppState, Claims, Message,
};

#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    body: web::Json<AuthUserSchema>,
) -> error::Result<Message> {
    if let Some(_) = sqlx::query!("SELECT * FROM users WHERE username = $1", body.username)
        .fetch_optional(&app_state.db)
        .await?
    {
        return Err(ServerError::Conflict(
            "Account with that username already exists",
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash_secret = app_state.config.hash_secret.as_bytes();

    let argon2 = Argon2::new_with_secret(
        hash_secret,
        Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    )
    .unwrap();

    let password_hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id",
        body.username,
        password_hash
    )
    .fetch_one(&app_state.db)
    .await?;

    Ok(Json(Message::new("Account created successfully")))
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub id: Uuid,
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    body: web::Json<AuthUserSchema>,
) -> error::Result<TokenResponse> {
    let user = match sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE username = $1",
        body.username
    )
    .fetch_optional(&app_state.db)
    .await?
    {
        None => {
            return Err(ServerError::NotFound(
                "Account with that username doesn't exist",
            ))
        }
        Some(user) => user,
    };

    let hash_secret = app_state.config.hash_secret.as_bytes();
    let argon2 = Argon2::new_with_secret(
        hash_secret,
        Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    )
    .unwrap();

    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    match argon2.verify_password(body.password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            let jwt_secret = app_state.config.jwt_secret.as_bytes();

            let exp = (Utc::now() + Duration::days(30)).timestamp();
            let claims = Claims {
                id: user.id.to_string(),
                exp,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_secret),
            )
            .unwrap();

            Ok(Json(TokenResponse { token, id: user.id }))
        }
        Err(_) => Err(ServerError::Unauthorized("Invalid password")),
    }
}

#[get("/me")]
pub async fn account_information(
    user: AuthUser,
    app_state: web::Data<AppState>,
) -> error::Result<UserModel> {
    let user = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", user.id)
        .fetch_one(&app_state.db)
        .await?;
    Ok(Json(user))
}
