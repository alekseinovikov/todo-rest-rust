use rocket_db_pools::{Connection, sqlx};
use rocket_db_pools::sqlx::{Postgres, Row};

use crate::service::entity::{CreateUpdateTodo, Id, Todo};
use crate::Todos;

pub struct Repository;

impl Repository {
    pub(crate) fn new() -> Repository {
        Repository
    }

    pub(crate) async fn list(&self, mut db: Connection<Todos>) -> Vec<Todo> {
        sqlx::query_as::<Postgres, Todo>("SELECT id, content, done FROM todo")
            .fetch_all(&mut *db)
            .await
            .unwrap()
    }

    pub(crate) async fn get_by_id(&self, id: Id, mut db: Connection<Todos>) -> Option<Todo> {
        sqlx::query_as::<Postgres, Todo>("SELECT id, content, done FROM todo WHERE id = $1")
            .bind(id as i64)
            .fetch_optional(&mut *db)
            .await
            .unwrap()
    }

    pub(crate) async fn update<'r>(&self, id: Id, todo: CreateUpdateTodo<'r>, mut db: Connection<Todos>) -> Result<Todo, String> {
        let _ = sqlx::query("UPDATE todo SET content = $1, done = $2 WHERE id = $3")
            .bind(todo.content)
            .bind(todo.done)
            .bind(id as i64)
            .execute(&mut *db)
            .await
            .map_err(|err| err.to_string())?;

        self.get_by_id(id, db).await
            .ok_or("The record is not found!".to_owned())
    }

    pub(crate) async fn create(&self, todo: CreateUpdateTodo<'_>, mut db: Connection<Todos>) -> Todo {
        let id: i64 = sqlx::query("INSERT INTO todo(content, done) VALUES($1, $2) RETURNING id")
            .bind(todo.content)
            .bind(todo.done)
            .fetch_one(& mut *db)
            .await
            .unwrap()
            .try_get("id")
            .unwrap();

        self.get_by_id(id as usize, db).await.unwrap()
    }
}