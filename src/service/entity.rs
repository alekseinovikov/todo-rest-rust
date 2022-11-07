use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::postgres::PgRow;
use rocket_db_pools::sqlx::{Error, Row};
use sqlx::FromRow;

pub(crate) type Id = usize;

#[derive(Serialize, Debug, PartialOrd, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub id: Id,
    pub content: String,
    pub done: bool,
}

impl<'r> FromRow<'r, PgRow> for Todo {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let id: i64 = row.try_get("id")?;
        let content = row.try_get("content")?;
        let done = row.try_get("done")?;

        Ok(Todo{ id: id as usize, content, done })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CreateUpdateTodo<'r> {
    pub content: &'r str,
    pub done: bool,
}
