use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub title: String,
    pub level: String,
    pub compensation: i32,
    pub joined_date: NaiveDate,
}
