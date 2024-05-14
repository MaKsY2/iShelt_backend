use crate::auth::auth;
use my_redis::controllers::user_controller::{UserController, UserControllerImpl};
use my_redis::establish_connection;
use my_redis::models::auth_models::{LoginRequest, LoginResponse};
use my_redis::models::user_model::{User, UserCreateModel};

extern crate rocket;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::*;

#[post("/auth", format = "json", data = "<data_json>")]
pub fn auth_user(data_json: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
    let mut connection = establish_connection();
    let user_controller: UserController = UserController();
    let user = user_controller.get_user(&mut connection, data_json.name.clone());
    print!("{}", user.is_ok());
    match user {
        Ok(user) => {
            return Ok(Json(LoginResponse {
                token: auth::create_jwt(user.id),
                user: user,
            }));
        }
        Err(_) => return Err(Status::NotFound),
    }
}

#[post("/create_user", format = "json", data = "<data_json>")]
pub fn create_user(data_json: Json<UserCreateModel>) -> Result<Json<bool>, Status> {
    let mut connection = establish_connection();
    let user_controller: UserController = UserController();
    let user = user_controller.create_user(&mut connection, data_json.0);

    match user {
        Ok(_) => return Ok(Json(true)),
        Err(_) => return Err(Status::NotFound),
    }
}
#[get("/all_users")]
pub fn get_all_users() -> Result<Json<Vec<User>>, Status> {
    let mut connection = establish_connection();
    let user_controller: UserController = UserController();
    let users = user_controller.get_users(&mut connection);

    match users {
        Ok(us) => return Ok(Json(us)),
        Err(_) => return Err(Status::NotFound),
    }
}
