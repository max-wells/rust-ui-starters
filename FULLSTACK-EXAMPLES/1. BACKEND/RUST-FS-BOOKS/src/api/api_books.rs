use leptos::*;
use reqwest::Client;
use web_sys::console;

use crate::models::model_books::{MyBook, BookId, NewBook};

const BASE_URL_BOOKS: &str = "http://localhost:8000/books";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/


// * 💁 From the client 💡
pub async fn get_all_books() -> Result<Vec<MyBook>, reqwest::Error> {
    let response = reqwest::get(BASE_URL_BOOKS).await?;
    let books: Vec<MyBook> = response.json().await?;
    
    Ok(books)
}

pub async fn get_book_by_id(id: BookId) -> Result<Option<MyBook>, ServerFnError> {
    let url = format!("{}/{}", BASE_URL_BOOKS, id);
    let response = reqwest::get(&url).await?;
    
    if response.status().is_success() {
        let book: MyBook = response.json().await?;
        Ok(Some(book))
    } else {
        Err(ServerFnError::ServerError(format!("Failed to retrieve book with ID: {}", id)))
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn delete_book(id: BookId) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let url = format!("{}/{}", BASE_URL_BOOKS, id); 
    // * 💁 I needed to implement fmt::Display for ServerBookId to use it directly 
    // *    and not have id.0 since it's a tuple

    let response = client.delete(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to delete book"))
    }
}


pub async fn add_book(new_book: NewBook) -> Result<MyBook, anyhow::Error> {
    let client = Client::new();
    let url = BASE_URL_BOOKS;

    // Use web_sys to log to the browser console
    console::log_1(&format!("new_book: {:?}", new_book).into());

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .json(&new_book)
        .send().await?;

    if response.status().is_success() {
        let book: MyBook = response.json().await?;
        Ok(book)
    } else {
        Err(anyhow::anyhow!("Failed to add book"))
    }
}
