#[macro_use]
extern crate rocket;
mod auth;

#[get("/")]
fn index() -> &'static str {
    "Hello, world"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/auth")]
fn auth() -> String {}

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello])
}
