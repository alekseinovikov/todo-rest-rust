use std::collections::HashMap;
use std::vec;

use rocket::tokio::sync::RwLock;

use crate::service::entity::{CreateUpdateTodo, Id, Todo};

pub struct Repository {
    todos: RwLock<HashMap<Id, Todo>>,
}

impl Repository {
    pub(crate) fn new() -> Repository {
        let vec = vec![
            Todo::new(1, "Wash hands".to_owned(), false),
            Todo::new(2, "Jerk off".to_owned(), true),
            Todo::new(3, "Sleep on work".to_owned(), false),
        ];

        let mut map = HashMap::new();
        vec.iter().for_each(|todo| {
            map.insert(todo.id, todo.to_owned());
        });

        Repository {
            todos: RwLock::new(map)
        }
    }

    pub(crate) async fn list(&self) -> Vec<Todo> {
        let todos = self.todos.read().await;
        let result: Vec<Todo> = todos.values().map(|todo| todo.clone()).collect();
        result
    }

    pub(crate) async fn get_by_id(&self, id: Id) -> Option<Todo> {
        let todos = self.todos.read().await;
        todos.get(&id).map(|todo| todo.clone())
    }

    pub(crate) async fn update<'r>(&self, id: Id, todo: CreateUpdateTodo<'r>) -> Result<Todo, &'static str> {
        let mut todos = self.todos.write().await;
        match todos.get_mut(&id) {
            None => Err("Not found"),
            Some(found) => {
                found.content = todo.content.to_owned();
                found.done = todo.done;

                Ok(found.clone())
            }
        }
    }

    pub(crate) async fn create(&self, todo: CreateUpdateTodo<'_>) -> Todo {
        let mut todos = self.todos.write().await;
        let new_id = todos.keys().max().unwrap_or(&1) + 1;

        let new_todo = Todo::new(new_id, todo.content.to_owned(), todo.done);
        let new_todo_copy = new_todo.clone();

        todos.insert(new_id, new_todo);

        new_todo_copy
    }
}