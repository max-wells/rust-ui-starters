use leptos::*;
use reqwest::Client;

use crate::models::model_tags::{MyTag, BlogId, NewTag};

const BASE_URL_BOOKS: &str = "http://localhost:8000/blogs/tags";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/


// * 💁 From the client 💡
pub async fn get_all_tags() -> Result<Vec<MyTag>, reqwest::Error> {
    let response = reqwest::get(BASE_URL_BOOKS).await?;
    let tags: Vec<MyTag> = response.json().await?;
    
    Ok(tags)
}

pub async fn get_tag_by_id(id: BlogId) -> Result<Option<MyTag>, ServerFnError> {
    let url = format!("{}/{}", BASE_URL_BOOKS, id);
    let response = reqwest::get(&url).await?;
    
    if response.status().is_success() {
        let tag: MyTag = response.json().await?;
        Ok(Some(tag))
    } else {
        Err(ServerFnError::ServerError(format!("Failed to retrieve tag with ID: {}", id)))
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn delete_tag(id: BlogId) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let url = format!("{}/{}", BASE_URL_BOOKS, id); 
    // * 💁 I needed to implement fmt::Display for ServerBlogId to use it directly 
    // *    and not have id.0 since it's a tuple

    let response = client.delete(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to delete tag"))
    }
}


pub async fn add_tag(new_tag: NewTag) -> Result<MyTag, anyhow::Error> {
    let client = Client::new();
    let url = BASE_URL_BOOKS;

    println!("[API] --> new_tag: {:?}", new_tag);

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .json(&new_tag)
        .send().await?;

    if response.status().is_success() {
        let tag: MyTag = response.json().await?;
        Ok(tag)
    } else {
        Err(anyhow::anyhow!("Failed to add tag"))
    }
}
