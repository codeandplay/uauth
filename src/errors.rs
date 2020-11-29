use rocket::http::Status;
use rocket::request::Request;
use rocket::response::status;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;

#[derive(Debug)]
pub struct RespError {
    pub error: String,
    pub code: Status,
}

impl RespError {
    pub fn new(code: Status, err: String) -> Self {
        Self { error: err, code }
    }
}

impl<'r> Responder<'r> for RespError {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        status::Custom(self.code, Json(json!({ "error": self.error }))).respond_to(req)
    }
}
