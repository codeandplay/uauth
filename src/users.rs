use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Logout {
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotPassword {
    email: String,
    #[serde(rename(deserialize = "recaptchaToken"))]
    recaptcha_token: String,
}

#[post("/register", data = "<user>")]
pub fn register(user: Json<User>) -> JsonValue {
    json!({
        "status": "oks"
    })
}

#[post("/login", data = "<login>")]
pub fn login(login: Json<Login>) -> JsonValue {
    json!({
        "token": "thisisthetoken"
    })
}

#[delete("/logout")]
pub fn logout() -> JsonValue {
    json!({
        "status": "ok"
    })
}

#[post("/forgot_password", data = "<forgot_password>")]
pub fn forgot_password(forgot_password: Json<ForgotPassword>) -> JsonValue {
    json!({
        "status": "ok"
    })
}
