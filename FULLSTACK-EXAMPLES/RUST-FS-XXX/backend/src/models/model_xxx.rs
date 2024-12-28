use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Xxx {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub field_one: String,
    pub field_two: String,
}
