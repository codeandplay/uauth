use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

use crate::users::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshToken {
    #[serde(rename(deserialize = "refreshToken"))]
    refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyToken {
    token: String,
}

#[post("/extend_token", data = "<refresh_token>")]
pub fn extend_token(refresh_token: Json<RefreshToken>) -> JsonValue {
    json!({
        "status": "ok"
    })
}

#[post("/verify_token", data = "<verify_token>")]
pub fn verify_token(verify_token: Json<VerifyToken>) -> Json<User> {
    Json(User {
        email: String::from("abc@gmail.com"),
        password: String::from("hello123"),
    })
}
