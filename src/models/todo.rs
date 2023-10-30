use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub user_id: i32,
}
