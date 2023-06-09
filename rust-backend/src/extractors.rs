use std::{
    future::{ready, Ready},
    ops,
    str::FromStr,
};

use actix_web::{
    dev::Payload, error, http::header, web, Error as ActixWebError, FromRequest, HttpRequest,
};
use jsonwebtoken::{decode, errors::ErrorKind as JWTError, DecodingKey, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{AppState, Claims};

/// Extractor that checks for and validates Bearer Token in the Authorization header
///
/// Sends UNAUTHORIZED response if token is invalid or expired
///
/// Extracts user id from the token
#[derive(Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
}

impl FromRequest for AuthUser {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let un_auth_err = ready(Err(error::ErrorUnauthorized(
            json!({"message": "Invalid auth token!"}),
        )));

        let token = match req.headers().get(header::AUTHORIZATION) {
            Some(val) => val.to_str().unwrap_or(""),
            None => "",
        };

        // Must contain Bearer keyword
        if token.len() < 7 {
            return un_auth_err;
        }

        let token = &token[7..];

        let jwt_secret = req
            .app_data::<web::Data<AppState>>()
            .unwrap()
            .config
            .jwt_secret
            .as_bytes();

        let decode = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret),
            &Validation::default(),
        );

        match decode {
            Ok(token) => ready(Ok(AuthUser {
                id: Uuid::from_str(&token.claims.id).unwrap(),
            })),
            Err(err) if err.kind() == &JWTError::ExpiredSignature => ready(Err(
                error::ErrorUnauthorized(json!({"message": "Auth token expired!"})),
            )),
            Err(_) => un_auth_err,
        }
    }
}

/// Custom Query params extractor
///
/// Create because default actix-web extractor doesn't support array types
#[derive(Debug)]
pub struct Query<T>(pub T);

impl<T: DeserializeOwned> FromRequest for Query<T> {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let query = req.query_string();

        let config = serde_qs::Config::new(20, false);
        let parsed = config.deserialize_str(query);

        match parsed {
            Err(error) => ready(Err(error::ErrorBadRequest(error))),
            Ok(parsed) => ready(Ok(Query(parsed))),
        }
    }
}

impl<T> ops::Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
