#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::response::status;
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Login {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Logout {
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RefreshToken {
    #[serde(rename(deserialize = "refreshToken"))]
    refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ForgotPassword {
    email: String,
    #[serde(rename(deserialize = "recaptchaToken"))]
    recaptcha_token: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct VerifyToken {
    token: String,
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/")]
fn index() -> &'static str {
    "It works"
}

#[post("/register", data = "<user>")]
fn register(user: Json<User>) -> JsonValue {
    json!({
        "status": "ok"
    })
}

#[post("/login", data = "<login>")]
fn login(login: Json<Login>) -> JsonValue {
    json!({
        "token": "thisisthetoken"
    })
}

#[delete("/logout")]
fn logout() -> JsonValue {
    json!({
        "status": "ok"
    })
}

#[post("/extend_token", data = "<refresh_token>")]
fn extend_token(refresh_token: Json<RefreshToken>) -> JsonValue {
    json!({
        "status": "ok"
    })
}

#[post("/forgot_password", data = "<forgot_password>")]
fn forgot_password(forgot_password: Json<ForgotPassword>) -> JsonValue {
    json!({
        "status": "ok"
    })
}

#[post("/verify_token", data = "<verify_token>")]
fn verify_token(verify_token: Json<VerifyToken>) -> Json<User> {
    Json(User {
        email: String::from("abc@gmail.com"),
        password: String::from("hello123"),
    })
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount(
            "/",
            routes![
                index,
                register,
                login,
                logout,
                extend_token,
                forgot_password,
                verify_token,
            ],
        )
        .launch();
}
