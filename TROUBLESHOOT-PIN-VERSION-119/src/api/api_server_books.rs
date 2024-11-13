use leptos::*;
use reqwest::Client;

use crate::models::model_server_books::{ServerBook, ServerBookId, NewServerBook};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{sync::RwLock};
        static GLOBAL_SERVER_BOOKS: RwLock<Vec<ServerBook>> = RwLock::new(vec![]);
    }
}


const URL_SERVER_BOOKS: &str = "http://localhost:8000/books";





/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[server(GetServerBook, "/api")]
pub async fn get_server_book(id: ServerBookId) -> Result<Option<ServerBook>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let books = GLOBAL_SERVER_BOOKS.read().unwrap();
    Ok(books.iter().find(|book| book.id == id).cloned())
}

#[server(GetServerBooks, "/api")]
pub async fn get_all_server_books() -> Result<Vec<ServerBook>, ServerFnError> {
    let response = reqwest::get(URL_SERVER_BOOKS).await?;
    let books: Vec<ServerBook> = response.json().await?;
    
    Ok(books)
}

// * 💁 From the client 💡
pub async fn get_all_server_books_from_client() -> Result<Vec<ServerBook>, reqwest::Error> {
    let response = reqwest::get(URL_SERVER_BOOKS).await?;
    let books: Vec<ServerBook> = response.json().await?;
    
    Ok(books)
}


/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/



// #[server(DeleteServerBook, "/api")]
// pub async fn delete_server_book(id: ServerBookId) -> Result<(), ServerFnError> {
//     tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
//     let mut server_books = GLOBAL_SERVER_BOOKS.write().unwrap();
//     server_books.retain(|sb| sb.id != id);
//     Ok(())
// }

pub async fn delete_server_book_from_client(id: ServerBookId) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let url = format!("{}/{}", URL_SERVER_BOOKS, id); 
    // * 💁 I needed to implement fmt::Display for ServerBookId to use it directly 
    // *    and not have id.0 since it's a tuple

    let response = client.delete(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to delete book"))
    }
}


#[server(AddServerBook, "/api")]
pub async fn add_server_book(title: String, author: String) -> Result<ServerBook, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let mut server_books = GLOBAL_SERVER_BOOKS.write().unwrap();

    let new_id = server_books
        .last()
        .map(|t| t.id.0 + 1)
        .map(ServerBookId)
        .unwrap_or(ServerBookId(0));

    let new_server_book = ServerBook {
        id: new_id,
        title,
        author,

    };

    server_books.push(new_server_book.clone());

    Ok(new_server_book)
}

//
//
pub async fn add_server_book_from_client(new_server_book: NewServerBook) -> Result<ServerBook, anyhow::Error> {
    let client = Client::new();
    let url = URL_SERVER_BOOKS;

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .json(&new_server_book)
        .send().await?;

    if response.status().is_success() {
        let server_book: ServerBook = response.json().await?;
        Ok(server_book)
    } else {
        Err(anyhow::anyhow!("Failed to add book"))
    }
}
