use serde::{Deserialize, Serialize};

use crate::features::auth::auth_models::User;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub user: Option<User>,
    pub title: String,
    pub created_at: String,
    pub completed: bool,
}
