use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_todos::{get_all_todos_url, get_todo_url, get_all_todos_from_client},
    models::model_users_url::{
        AllTodosTagUrl, AllTodosTagUrlFromClient, UserIdUrl, TodoResponseUrl, MyTodo,
    },
};

// TODO. Create a [#hook] for this

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(non_snake_case)]
pub fn useUsersQueryUrl() -> QueryScope<UserIdUrl, TodoResponseUrl> {
    create_query(
        get_todo_url,
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}

#[allow(non_snake_case)]
pub fn useAllTodosQueryUrl() -> QueryScope<AllTodosTagUrl, Vec<MyTodo>> {
    create_query(
        |_| async move { get_all_todos_url().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}


#[allow(non_snake_case)]
pub fn useAllTodosFromClient() -> QueryScope<AllTodosTagUrlFromClient, Vec<MyTodo>> {
    create_query(
        |_| async move { get_all_todos_from_client().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}
