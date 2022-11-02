use rocket::Request;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize, Debug, PartialOrd, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Error {
    message: &'static str,
    error_code: u16,
}

const NOT_FOUND_ERROR: &'static Error = &Error {
    message: "Not found",
    error_code: 404,
};

const INTERNAL_SERVER_ERROR: &'static Error = &Error {
    message: "Internal server error",
    error_code: 500,
};

#[catch(404)]
pub(crate) fn not_found(_req: &Request) -> Json<&'static Error> {
    Json(NOT_FOUND_ERROR)
}

#[catch(500)]
pub(crate) fn internal_server_error(_req: &Request) -> Json<&'static Error> {
    Json(INTERNAL_SERVER_ERROR)
}

