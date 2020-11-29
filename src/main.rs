#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;

use rocket::fairing::AdHoc;
use rocket::response::status;
use rocket::Request;
use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::Cors;
use serde::{Deserialize, Serialize};

pub mod authentication;
pub mod config;
pub mod crypto;
pub mod errors;
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

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

fn main() {
    rocket::custom(config::from_env())
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
        .attach(models::Conn::fairing())
        .attach(AdHoc::on_attach(
            "Database Migrations",
            models::run_db_migrations,
        ))
        .attach(cors_fairing())
        .launch();
}
