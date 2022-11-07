use std::collections::HashMap;
use std::vec;

use rocket::tokio::sync::RwLock;
use rocket_db_pools::{Database, Connection, sqlx};
use rocket_db_pools::sqlx::postgres::PgRow;
use rocket_db_pools::sqlx::Row;

use crate::service::entity::{CreateUpdateTodo, Id, Todo};
use crate::Todos;

pub struct Repository;

impl Repository {
    pub(crate) fn new() -> Repository {
        Repository
    }

    pub(crate) async fn list(&self, mut db: Connection<Todos>) -> Vec<Todo> {
        vec![]
    }

    pub(crate) async fn get_by_id(&self, id: Id, mut db: Connection<Todos>) -> Option<Todo> {
        sqlx::query_as("SELECT id, content, done FROM todo WHERE id = $1")
            .bind(id as i64)
            .map(|row: PgRow| {
                let id: i64 = row.try_get("id").unwrap();
                let content = row.try_get("content").unwrap();
                let done = row.try_get("done").unwrap();
                Todo {id: id as usize, content, done}
            }).fetch_optional(&mut *db).await
    }

    pub(crate) async fn update<'r>(&self, id: Id, todo: CreateUpdateTodo<'r>, mut db: Connection<Todos>) -> Result<Todo, &'static str> {
        Ok(Todo::new(1, format!(""), true))
    }

    pub(crate) async fn create(&self, todo: CreateUpdateTodo<'_>, mut db: Connection<Todos>) -> Todo {
        Todo::new(1, format!(""), true)
    }
}