use crate::service::entity::{CreateUpdateTodo, Id, Todo};
use crate::service::repository::Repository;
use rocket_db_pools::Connection;
use crate::Todos;

pub(crate) mod entity;
pub(crate) mod repository;

pub(crate) struct TodoService {
    repository: Repository,
}

impl TodoService {
    pub(crate) fn new() -> TodoService {
        TodoService {
            repository: Repository::new()
        }
    }

    pub(crate) async fn list(&self,
                             db: Connection<Todos>) -> Vec<Todo> {
        self.repository.list(db).await
    }

    pub(crate) async fn get_by_id(&self,
                                  id: Id,
                                  db: Connection<Todos>) -> Option<Todo> {
        self.repository.get_by_id(id, db).await
    }

    pub(crate) async fn update<'r>(&self,
                               id: Id,
                               todo: CreateUpdateTodo<'r>,
                               db: Connection<Todos>) -> Result<Todo, String> {
        self.repository.update(id, todo, db).await
    }

    pub(crate) async fn create(&self,
                               todo: CreateUpdateTodo<'_>,
                               db: Connection<Todos>) -> Todo {
        self.repository.create(todo, db).await
    }
}