use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoBody {
    pub title: String,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoBody {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
