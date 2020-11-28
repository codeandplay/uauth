#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::response::status;
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

mod authentication;
mod users;

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
