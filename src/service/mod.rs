use crate::service::entity::{CreateUpdateTodo, Id, Todo};
use crate::service::repository::Repository;

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

    pub(crate) async fn list(&self) -> Vec<Todo> {
        self.repository.list().await
    }

    pub(crate) async fn get_by_id(&self, id: Id) -> Option<Todo> {
        self.repository.get_by_id(id).await
    }

    pub(crate) async fn update<'r>(&self, id: Id, todo: CreateUpdateTodo<'r>) -> Result<Todo, &'static str> {
        self.repository.update(id, todo).await
    }

    pub(crate) async fn create(&self, todo: CreateUpdateTodo<'_>) -> Todo {
        self.repository.create(todo).await
    }
}