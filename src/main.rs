#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use crate::service::entity::Todo;
use crate::service::TodoService;

mod service;


#[get("/")]
fn list(service: &State<TodoService>) -> Json<Vec<&Todo>> {
    Json(service.list())
}

#[get("/<id>")]
fn get(id: usize, service: &State<TodoService>) -> Option<Json<&Todo>> {
    service.get_by_id(id).map(|x| Json(x))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TodoService::new())
        .mount("/", routes![
            list,
            get
        ])
}
