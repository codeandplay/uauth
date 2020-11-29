use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshToken {
    #[serde(rename(deserialize = "refreshToken"))]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyToken {
    pub token: String,
}

/// /extend_token handler
#[post("/extend_token", data = "<refresh_token>")]
pub fn extend_token(refresh_token: Json<RefreshToken>) -> JsonValue {
    json!({
        "status": "ok"
    })
}

/// verify_token handler
#[post("/verify_token", data = "<verify_token>")]
pub fn verify_token(verify_token: Json<VerifyToken>) -> JsonValue {
    json!({
        "status": "ok"
    })
    //Json(User {
    //    email: String::from("abc@gmail.com"),
    //    password: String::from("hello123"),
    //})
}
