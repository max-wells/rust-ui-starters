use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
}
