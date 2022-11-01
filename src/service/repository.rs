use std::collections::HashMap;
use crate::service::entity::{Id, Todo};

pub struct Repository {
    todos: HashMap<Id, Todo>,
}

impl Repository {
    pub(crate) fn new() -> Repository {
        let vec = vec![
            Todo::new(1, "Wash hands".to_owned(), false),
            Todo::new(2, "Jerk off".to_owned(), true),
            Todo::new(3, "Sleep on work".to_owned(), false)
        ];

        let mut map = HashMap::new();
        vec.iter().for_each(|todo| {
            map.insert(todo.id, todo.to_owned());
        });

        Repository {
            todos: map
        }
    }

    pub(crate) fn list(&self) -> Vec<&Todo> {
        self.todos.values().collect()
    }

    pub(crate) fn get_by_id(&self, id: Id) -> Option<&Todo> {
        self.todos.get(&id)
    }
}