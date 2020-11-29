use crate::schema::users;
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

#[derive(Queryable)]
pub struct SessionKey {
    pub id: Uuid,
    pub userid: Uuid,
    pub key: String,
    pub expiry: DateTime<Utc>,
    pub extended_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub fail_logins: Option<i32>,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(
    conn: &PgConnection,
    email: String,
    password_hash: &'a str,
    password_salt: &'a str,
) -> User {
    let new_user = NewUser {
        email,
        password_hash: password_hash.to_string(),
        password_salt: password_salt.to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}
