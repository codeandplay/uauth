use crate::crypto::gen_session_key;
use crate::schema::{session_keys, users};
use chrono::prelude::*;
use chrono::Duration;
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket_contrib::databases::diesel;
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

#[derive(Insertable)]
#[table_name = "session_keys"]
pub struct NewSessionKey {
    pub userid: Uuid,
    pub key: String,
    pub expiry: DateTime<Utc>,
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

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);

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
) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        email,
        password_hash: password_hash.to_string(),
        password_salt: password_salt.to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn new_session_key<'a>(
    conn: &PgConnection,
    userid: Uuid,
) -> Result<SessionKey, diesel::result::Error> {
    let key = gen_session_key().unwrap();
    let expiry = chrono::offset::Utc::now() + Duration::minutes(10);
    let new_session = NewSessionKey {
        userid,
        key,
        expiry,
    };

    diesel::insert_into(session_keys::table)
        .values(&new_session)
        .get_result(conn)
}

pub fn get_user_by_email<'a>(
    conn: &PgConnection,
    email: String,
) -> Result<User, diesel::result::Error> {
    users::table
        .filter(
            users::email.eq(email).and(
                users::locked_until
                    .lt(chrono::offset::Utc::now())
                    .or(users::locked_until.is_null()),
            ),
        )
        .first(conn)
}
