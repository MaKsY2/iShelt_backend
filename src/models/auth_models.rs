use crate::models::user_model::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub uid: i32,
    pub exp: i64,
}
