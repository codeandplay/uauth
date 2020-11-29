#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use rocket::response::status;
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

pub mod authentication;
pub mod crypto;
pub mod models;
pub mod schema;
pub mod users;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/")]
fn index() -> &'static str {
    "It works"
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount(
            "/",
            routes![
                index,
                users::register,
                users::login,
                users::logout,
                users::forgot_password,
                authentication::extend_token,
                authentication::verify_token,
            ],
        )
        .launch();
}
