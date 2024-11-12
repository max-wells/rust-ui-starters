use leptos::*;

use crate::models::model_users_json::{UserIdJson, UserJson};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{sync::RwLock};
        static GLOBAL_USERS: RwLock<Vec<UserJson>> = RwLock::new(vec![]);
    }
}





/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[server(GetUserJson, "/api")]
pub async fn get_user_json(id: UserIdJson) -> Result<Option<UserJson>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let users = GLOBAL_USERS.read().unwrap();
    Ok(users.iter().find(|user| user.id == id).cloned())
}

#[server(GetUsersJson, "/api")]
pub async fn get_all_users_json() -> Result<Vec<UserJson>, ServerFnError> {
    // Fetch todos from the external URL
    let response = reqwest::get("http://localhost:3000/users.json").await?;
    let users: Vec<UserJson> = response.json().await?;
    
    Ok(users)
}
