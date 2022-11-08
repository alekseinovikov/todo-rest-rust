#[macro_use]
extern crate rocket;

use prometheus::{self, IntCounter};

use rocket::{State};
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};
use rocket_db_pools::sqlx::{self};
use lazy_static::lazy_static;
use prometheus::register_int_counter;


use crate::service::entity::*;
use crate::service::TodoService;

mod service;
mod errors;
mod migrations;


lazy_static! {
    static ref LIST_COUNTER: IntCounter =
        register_int_counter!("list", "Counter of list requests").unwrap();
    static ref GET_COUNTER: IntCounter =
        register_int_counter!("get", "Counter of get requests").unwrap();
    static ref CREATE_COUNTER: IntCounter =
        register_int_counter!("create", "Counter of create requests").unwrap();
    static ref UPDATE_COUNTER: IntCounter =
        register_int_counter!("update", "Counter of update requests").unwrap();
}

rocket_healthz::healthz!();

#[get("/")]
async fn list(service: &State<TodoService>,
              db: Connection<Todos>) -> Json<Vec<Todo>> {
    LIST_COUNTER.inc();
    Json(service
        .list(db)
        .await)
}

#[get("/<id>")]
async fn get(id: usize,
             service: &State<TodoService>,
             db: Connection<Todos>) -> Option<Json<Todo>> {
    GET_COUNTER.inc();
    service
        .get_by_id(id, db)
        .await
        .map(|x| Json(x))
}

#[put("/<id>", data = "<todo>")]
async fn update<'r>(id: usize,
                    todo: Json<CreateUpdateTodo<'r>>,
                    service: &State<TodoService>,
                    db: Connection<Todos>) -> Result<Json<Todo>, String> {
    UPDATE_COUNTER.inc();
    service
        .update(id, todo.0, db)
        .await
        .map(|todo| Json(todo))
}

#[post("/", data = "<todo>")]
async fn create<'r>(todo: Json<CreateUpdateTodo<'r>>,
                    service: &State<TodoService>,
                    db: Connection<Todos>) -> Json<Todo> {
    CREATE_COUNTER.inc();
    Json(service
        .create(todo.0, db)
        .await)
}

#[derive(Database)]
#[database("pg_todo")]
struct Todos(sqlx::PgPool);

#[launch]
async fn rocket() -> _ {
    let rocket = rocket::build();

    migrations::migrate(&rocket).await;

    prometheus_exporter::start("0.0.0.0:9090".parse().expect(
        "failed to parse binding",
    )).expect("failed to start prometheus exporter");

    rocket.attach(Todos::init())
        .register("/", catchers![
            errors::not_found,
            errors::internal_server_error
        ]).manage(TodoService::new())
        .mount("/", routes![
            list,
            get,
            update,
            create,

            healthz
        ])
}
