use rocket::serde::{Serialize, Deserialize};

pub(crate) type Id = usize;

#[derive(Serialize, Debug, PartialOrd, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub id: Id,
    pub content: String,
    pub done: bool
}

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CreateUpdateTodo<'r> {
    pub content: &'r str,
    pub done: bool
}

impl Todo {
    pub(crate) fn new(id: Id, content: String, done: bool) -> Todo {
        Todo {id, content, done}
    }
}