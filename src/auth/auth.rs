use chrono;
use jsonwebtoken as jwt;
use jwt::{decode, errors::Error, Algorithm, DecodingKey, Validation};
use std::env;

use my_redis::models::auth_models::Claims;

pub fn create_jwt(uid: i32) -> String {
    let secret_key: String = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let experation = chrono::Utc::now().timestamp() + 3600;
    let claims = Claims {
        uid: uid,
        exp: experation,
    };
    let token = jwt::encode(
        &jwt::Header::default(),
        &claims,
        &jwt::EncodingKey::from_secret(secret_key.as_ref()),
    )
    .unwrap();
    token
}

pub fn extract_jwt(token: String) -> Result<Claims, Error> {
    return match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(
            env::var("SECRET_KEY")
                .expect("SECRET_KEY must be set")
                .as_ref(),
        ),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => Err(err),
    };
}
