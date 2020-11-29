use crate::crypto::gen_session_key;
use crate::schema::{session_keys, users};
use chrono::prelude::*;
use chrono::Duration;
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::Rocket;
use rocket_contrib::databases::diesel;
use std::env;
use uuid::Uuid;

// in minutes
const SESSION_KEY_DURATION: i64 = 30;

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

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

pub fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = Conn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
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
    let expiry = chrono::offset::Utc::now() + Duration::minutes(SESSION_KEY_DURATION);
    let new_session = NewSessionKey {
        userid,
        key,
        expiry,
    };

    diesel::insert_into(session_keys::table)
        .values(&new_session)
        .get_result(conn)
}

pub fn extend_session_key<'a>(
    conn: &PgConnection,
    key_id: Uuid,
) -> Result<usize, diesel::result::Error> {
    let new_expiry = chrono::offset::Utc::now() + Duration::minutes(SESSION_KEY_DURATION);
    diesel::update(session_keys::table.find(key_id))
        .set(session_keys::expiry.eq(new_expiry))
        .execute(conn)
}

pub fn delete_session_key<'a>(
    conn: &PgConnection,
    key_id: Uuid,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(session_keys::table.find(key_id)).execute(conn)
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
