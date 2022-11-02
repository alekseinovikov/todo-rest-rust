#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::{Database, Connection};

use crate::service::TodoService;
use crate::service::entity::*;

mod service;
mod errors;

#[get("/")]
async fn list(service: &State<TodoService>,
              mut db: Connection<Todos>) -> Json<Vec<Todo>> {
    Json(service
        .list(db)
        .await)
}

#[get("/<id>")]
async fn get(id: usize,
             service: &State<TodoService>,
             mut db: Connection<Todos>) -> Option<Json<Todo>> {
    service
        .get_by_id(id, db)
        .await
        .map(|x| Json(x))
}

#[put("/<id>", data="<todo>")]
async fn update<'r>(id: usize,
                    todo: Json<CreateUpdateTodo<'r>>,
                    service: &State<TodoService>,
                    mut db: Connection<Todos>) -> Result<Json<Todo>, &'static str> {
    service
        .update(id, todo.0, db)
        .await
        .map(|todo| Json(todo))
}

#[post("/", data="<todo>")]
async fn create<'r>(todo: Json<CreateUpdateTodo<'r>>,
                    service: &State<TodoService>,
                    mut db: Connection<Todos>) -> Json<Todo> {
    Json(service
        .create(todo.0, db)
        .await)
}

#[derive(Database)]
#[database("pg_todo")]
struct Todos(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![
            errors::not_found,
            errors::internal_server_error
        ]).manage(TodoService::new())
        .attach(Todos::init())
        .mount("/", routes![
            list,
            get,
            update,
            create
        ])
}
