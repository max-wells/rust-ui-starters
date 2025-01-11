use leptos::*;
use reqwest::Client;

use crate::models::model_blogs::{MyBlog, BlogId, NewBlog};

const BASE_URL_BOOKS: &str = "http://localhost:8000/blogs";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/


// * 💁 From the client 💡
pub async fn get_all_blogs() -> Result<Vec<MyBlog>, reqwest::Error> {
    let response = reqwest::get(BASE_URL_BOOKS).await?;
    let blogs: Vec<MyBlog> = response.json().await?;
    
    Ok(blogs)
}

pub async fn get_blog_by_id(id: BlogId) -> Result<Option<MyBlog>, ServerFnError> {
    let url = format!("{}/{}", BASE_URL_BOOKS, id);
    let response = reqwest::get(&url).await?;
    
    if response.status().is_success() {
        let blog: MyBlog = response.json().await?;
        Ok(Some(blog))
    } else {
        Err(ServerFnError::ServerError(format!("Failed to retrieve blog with ID: {}", id)))
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn delete_blog(id: BlogId) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let url = format!("{}/{}", BASE_URL_BOOKS, id); 
    // * 💁 I needed to implement fmt::Display for ServerBlogId to use it directly 
    // *    and not have id.0 since it's a tuple

    let response = client.delete(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to delete blog"))
    }
}


pub async fn add_blog(new_blog: NewBlog) -> Result<MyBlog, anyhow::Error> {
    let client = Client::new();
    let url = BASE_URL_BOOKS;

    println!("[API] --> new_blog: {:?}", new_blog);

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .json(&new_blog)
        .send().await?;

    if response.status().is_success() {
        let blog: MyBlog = response.json().await?;
        Ok(blog)
    } else {
        Err(anyhow::anyhow!("Failed to add blog"))
    }
}
