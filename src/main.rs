#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use crate::service::entity::Todo;
use crate::service::TodoService;

mod service;

use rocket::{Catcher, Request};
use rocket::response::{Result, Responder};
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::serde::Serialize;

fn handle_404<'r>(req: &'r Request) -> Result<'r> {
    let res = Custom(Status::NotFound, format!("404: {}", req.uri()));
    res.respond_to(req)
}


#[get("/")]
fn list(service: &State<TodoService>) -> Json<Vec<&Todo>> {
    Json(service.list())
}

#[get("/<id>")]
fn get(id: usize, service: &State<TodoService>) -> Option<Json<&Todo>> {
    service.get_by_id(id).map(|x| Json(x))
}

#[derive(Serialize, Debug, PartialOrd, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
struct Error {
    message: &'static str,
    error_code: u16,
}

const not_found_error: &'static Error = &Error {
    message: "Idi nahuy",
    error_code: 404,
};

const not_found_json: &'static Json<&Error> = &Json(not_found_error);

#[catch(404)]
fn not_found(req: &Request) -> Json<&'static Error> {
    Json(not_found_error)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .manage(TodoService::new())
        .mount("/", routes![
            list,
            get
        ])
}
