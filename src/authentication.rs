use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

use crate::{errors::RespError, models::Conn};

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
pub fn extend_token(
    refresh_token: Json<RefreshToken>,
    conn: Conn,
) -> Result<status::NoContent, RespError> {
    // Return session key.
    //extend_token(&conn, user.id)
    //    .map(|session| {
    //        json!({
    //            "sessionId": session.id,
    //            "sessionKey":session.key,
    //            "expiry": session.expiry.timestamp(),
    //        })
    //    })
    //    .map_err(|err| match err {
    //        _ => {
    //            print!("Login error: {:?}", err);
    //            RespError::new(Status::BadRequest, Status::BadRequest.reason.to_string())
    //        }
    //    })

    Ok(status::NoContent)
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
