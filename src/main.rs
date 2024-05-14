extern crate rocket;

mod auth;
mod handlers;

use crate::handlers::user_handlers::{auth_user, create_user, get_all_users};

#[rocket::launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    rocket::build().mount("/", rocket::routes![auth_user, create_user, get_all_users])
}
