use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: String,
    pub image_url: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlogResponse {
    pub blog: BlogWithParsedTags,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBlogBody {
    pub title: String,
    pub content: String,
    pub author: String,
    pub image_url: Option<String>,
    pub tags: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize)]
pub struct BodyPatchBlog {
    pub title: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub image_url: Option<String>,
    pub tags: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogWithParsedTags {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: String,
    pub image_url: Option<String>,
    pub tags: Vec<i32>,
}

#[derive(Deserialize)]
pub struct CreateTag {
    pub name: String,
}
