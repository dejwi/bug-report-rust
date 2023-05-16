use std::future::{ready, Ready};

use actix_web::{
    dev::Payload, error, http::header, Error as ActixWebError, FromRequest, HttpRequest,
};
use jsonwebtoken::{decode, errors::ErrorKind as JWTError, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::Claims;

#[derive(Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
}

impl FromRequest for AuthUser {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let un_auth_err = ready(Err(error::ErrorUnauthorized("Invalid auth token!")));

        let token = match req.headers().get(header::AUTHORIZATION) {
            Some(val) => val.to_str().unwrap_or(""),
            None => "",
        };

        // Must contain Bearer keyword
        if token.is_empty() || token.len() < 7 {
            return un_auth_err;
        }

        let token = &token[7..];

        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let decode = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        );

        match decode {
            Ok(token) => ready(Ok(AuthUser {
                id: token.claims.id,
            })),
            Err(err) if err.kind() == &JWTError::ExpiredSignature => {
                ready(Err(error::ErrorUnauthorized("Auth token expired!")))
            }
            Err(_) => un_auth_err,
        }
    }
}
