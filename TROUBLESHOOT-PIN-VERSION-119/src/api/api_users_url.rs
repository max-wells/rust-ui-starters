use leptos::*;

use crate::models::model_users_url::{UserIdUrl, UserUrl};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{sync::RwLock};
        static GLOBAL_USERS: RwLock<Vec<UserUrl>> = RwLock::new(vec![]);
    }
}


const URL_USERS: &str = "https://jsonplaceholder.typicode.com/posts";





/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[server(GetUserUrl, "/api")]
pub async fn get_user_url(id: UserIdUrl) -> Result<Option<UserUrl>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let users = GLOBAL_USERS.read().unwrap();
    Ok(users.iter().find(|user| user.id == id).cloned())
}

#[server(GetUsersUrl, "/api")]
pub async fn get_all_users_url() -> Result<Vec<UserUrl>, ServerFnError> {
    let response = reqwest::get(URL_USERS).await?;
    let users: Vec<UserUrl> = response.json().await?;
    
    Ok(users)
}

// * 💁 From the client 💡
pub async fn get_all_users_url_from_client() -> Result<Vec<UserUrl>, reqwest::Error> {
    let response = reqwest::get(URL_USERS).await?;
    let users: Vec<UserUrl> = response.json().await?;
    
    Ok(users)
}