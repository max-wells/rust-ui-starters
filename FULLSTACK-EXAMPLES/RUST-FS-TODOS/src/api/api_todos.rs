use leptos::*;

use crate::models::model_users_url::{UserIdUrl, MyTodo};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{sync::RwLock};
        static GLOBAL_USERS: RwLock<Vec<MyTodo>> = RwLock::new(vec![]);
    }
}


const URL_TODOS: &str = "http://localhost:8000/todos";





/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[server(GetUserUrl, "/api")]
pub async fn get_todo_url(id: UserIdUrl) -> Result<Option<MyTodo>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let users = GLOBAL_USERS.read().unwrap();
    Ok(users.iter().find(|user| user.id == id).cloned())
}

#[server(GetUsersUrl, "/api")]
pub async fn get_all_todos_url() -> Result<Vec<MyTodo>, ServerFnError> {
    let response = reqwest::get(URL_TODOS).await?;
    let users: Vec<MyTodo> = response.json().await?;
    
    Ok(users)
}

// * 💁 From the client 💡
pub async fn get_all_todos_from_client() -> Result<Vec<MyTodo>, reqwest::Error> {
    let response = reqwest::get(URL_TODOS).await?;
    let users: Vec<MyTodo> = response.json().await?;
    
    Ok(users)
}