use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

use crate::{crypto::getRandomSalt, models::*};
use diesel::prelude::*;

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
pub fn register(user_register: Json<UserRegister>) -> JsonValue {
    let connection = establish_connection();
    //use crate::schema::users::dsl::*;
    //let results = users
    //    .limit(5)
    //    .load::<User>(&connection)
    //    .expect("Error loading posts");

    //println!("Displaying {} users", results.len());
    //for user in results {
    //    println!("{}", user.id);
    //    println!("----------\n");
    //    println!("{}", user.password_hash);
    //}
    //
    //
    // Generate password salt.
    let password_salt = getRandomSalt().unwrap();
    // Generate password hash from password + salt.
    let password_hash = "";
    create_user(
        &connection,
        user_register.email.clone(),
        password_hash,
        &password_salt,
    );
    json!({
        "status": "oks"
    })
}

/// /login handler
#[post("/login", data = "<login>")]
pub fn login(login: Json<Login>) -> JsonValue {
    json!({
        "token": "thisisthetoken"
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
