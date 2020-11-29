use crate::crypto::verify_password;
use std::io;

use chrono::Duration;
use rocket::{http::Status, response::status};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

use crate::{
    crypto::gen_session_key,
    crypto::{get_password_hash, get_random_salt},
    errors::RespError,
    models::*,
};
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::*;
use diesel::result::Error::DatabaseError;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRegister {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Logout {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotPassword {
    pub email: String,
    #[serde(rename(deserialize = "recaptchaToken"))]
    pub recaptcha_token: String,
}

/// /register handler
#[post("/register", data = "<user_register>")]
pub fn register(user_register: Json<UserRegister>, conn: Conn) -> Result<JsonValue, RespError> {
    // Generate password salt.
    let password_salt = match get_random_salt() {
        Ok(v) => v,
        Err(_) => {
            print!("Fail to create Random salt");
            return Err(RespError::new(
                Status::InternalServerError,
                Status::InternalServerError.to_string(),
            ));
        }
    };
    // Generate password hash from password + salt.
    let password_hash = match get_password_hash(&user_register.password, &password_salt) {
        Ok(v) => v,
        Err(_) => {
            print!("Fail to create Random salt");
            return Err(RespError::new(
                Status::InternalServerError,
                Status::InternalServerError.to_string(),
            ));
        }
    };
    create_user(
        &conn,
        user_register.email.clone(),
        &password_hash,
        &password_salt,
    )
    .map(|user| {
        let session = new_session_key(&conn, user.id).unwrap();

        json!({
            "sessionId": session.id,
            "sessionKey":session.key,
            "expiry": session.expiry.timestamp(),
        })
    })
    .map_err(|err| match err {
        diesel::result::Error::DatabaseError(UniqueViolation, _) => {
            RespError::new(Status::BadRequest, Status::BadRequest.reason.to_string())
        }
        _ => RespError::new(
            Status::InternalServerError,
            Status::InternalServerError.to_string(),
        ),
    })
}

/// /login handler
#[post("/login", data = "<login>")]
pub fn login(login: Json<Login>, conn: Conn) -> Result<JsonValue, RespError> {
    // fetch the user by email from database.
    let user = match get_user_by_email(&conn, login.email.clone()) {
        Ok(v) => v,
        Err(_) => {
            print!("Login with unknown email");
            return Err(RespError::new(
                Status::Unauthorized,
                Status::Unauthorized.reason.to_string(),
            ));
        }
    };

    // Verify the password.
    let hash = user.password_hash;
    let salt = user.password_salt;
    let cred_ok = match verify_password(&login.password, &hash, &salt) {
        Ok(_) => {
            println!("password ok");
            true
        }
        Err(err) => {
            println!("password not ok {:?}", err);
            false
        }
    };

    if !cred_ok {
        return Err(RespError::new(
            Status::Unauthorized,
            Status::Unauthorized.reason.to_string(),
        ));
    }

    // Return session key.
    new_session_key(&conn, user.id)
        .map(|session| {
            json!({
                "sessionId": session.id,
                "sessionKey":session.key,
                "expiry": session.expiry.timestamp(),
            })
        })
        .map_err(|err| match err {
            _ => {
                print!("Login error: {:?}", err);
                RespError::new(Status::BadRequest, Status::BadRequest.reason.to_string())
            }
        })
}

/// /logout handler
#[delete("/logout")]
pub fn logout() -> JsonValue {
    json!({
        "status": "ok"
    })
}

/// /forgot_password handler
#[post("/forgot_password", data = "<forgot_password>")]
pub fn forgot_password(forgot_password: Json<ForgotPassword>) -> JsonValue {
    json!({
        "status": "ok"
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(23 + 2, 25);
    }
}
