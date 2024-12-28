use leptos::*;
use reqwest::Client;
use web_sys::console;

use crate::models::model_xxxs::{MyXxx, XxxId, NewXxx};

const BASE_URL_BOOKS: &str = "http://localhost:8000/xxxs";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/


// * 💁 From the client 💡
pub async fn get_all_xxxs() -> Result<Vec<MyXxx>, reqwest::Error> {
    let response = reqwest::get(BASE_URL_BOOKS).await?;
    let xxxs: Vec<MyXxx> = response.json().await?;
    
    Ok(xxxs)
}

pub async fn get_xxx_by_id(id: XxxId) -> Result<Option<MyXxx>, ServerFnError> {
    let url = format!("{}/{}", BASE_URL_BOOKS, id);
    let response = reqwest::get(&url).await?;
    
    if response.status().is_success() {
        let xxx: MyXxx = response.json().await?;
        Ok(Some(xxx))
    } else {
        Err(ServerFnError::ServerError(format!("Failed to retrieve xxx with ID: {}", id)))
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn delete_xxx(id: XxxId) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let url = format!("{}/{}", BASE_URL_BOOKS, id); 
    // * 💁 I needed to implement fmt::Display for ServerBookId to use it directly 
    // *    and not have id.0 since it's a tuple

    let response = client.delete(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to delete xxx"))
    }
}


pub async fn add_xxx(new_xxx: NewXxx) -> Result<MyXxx, anyhow::Error> {
    let client = Client::new();
    let url = BASE_URL_BOOKS;

    // Use web_sys to log to the browser console
    console::log_1(&format!("new_xxx: {:?}", new_xxx).into());

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .json(&new_xxx)
        .send().await?;

    if response.status().is_success() {
        let xxx: MyXxx = response.json().await?;
        Ok(xxx)
    } else {
        Err(anyhow::anyhow!("Failed to add xxx"))
    }
}
