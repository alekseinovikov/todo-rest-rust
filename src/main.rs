#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;

use crate::service::entity::{CreateUpdateTodo, Todo};
use crate::service::TodoService;

mod service;
mod errors;

#[get("/")]
async fn list(service: &State<TodoService>) -> Json<Vec<Todo>> {
    Json(service.list().await)
}

#[get("/<id>")]
async fn get(id: usize, service: &State<TodoService>) -> Option<Json<Todo>> {
    service.get_by_id(id).await
        .map(|x| Json(x))
}

#[put("/<id>", data="<todo>")]
async fn update<'r>(id: usize, todo: Json<CreateUpdateTodo<'r>>, service: &State<TodoService>) -> Result<Json<Todo>, &'static str> {
    service.update(id, todo.0).await
        .map(|todo| Json(todo))
}

#[post("/", data="<todo>")]
async fn create<'r>(todo: Json<CreateUpdateTodo<'r>>, service: &State<TodoService>) -> Json<Todo> {
    Json(service.create(todo.0).await)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![
            errors::not_found,
            errors::internal_server_error
        ]).manage(TodoService::new())
        .mount("/", routes![
            list,
            get,
            update,
            create
        ])
}
