use crate::service::entity::{Id, Todo};
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

    pub(crate) fn list(&self) -> Vec<&Todo> {
        self.repository.list()
    }

    pub(crate) fn get_by_id(&self, id: Id) -> Option<&Todo> {
        self.repository.get_by_id(id)
    }
}