use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_todos_json::{get_all_todos_json, get_todo_json},
    models::model_todo_json::{AllTodosTagJson, TodoIdJson, TodoResponseJson, TodoJson},
};

// TODO. Create a [#hook] for this

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(non_snake_case)]
pub fn useTodoQueryJson() -> QueryScope<TodoIdJson, TodoResponseJson> {
    create_query(
        get_todo_json,
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}

#[allow(non_snake_case)]
pub fn useAllTodosQueryJson() -> QueryScope<AllTodosTagJson, Vec<TodoJson>> {
    create_query(
        |_| async move { get_all_todos_json().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}
