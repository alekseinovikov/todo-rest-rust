use rocket::serde::{Serialize};

pub(crate) type Id = usize;

#[derive(Serialize, Debug, PartialOrd, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub id: Id,
    pub content: String,
    pub done: bool
}

impl Todo {
    pub(crate) fn new(id: Id, content: String, done: bool) -> Todo {
        Todo {id, content, done}
    }
}