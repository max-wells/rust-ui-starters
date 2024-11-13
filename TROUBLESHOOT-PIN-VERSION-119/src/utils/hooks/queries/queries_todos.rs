use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_todos::{get_all_todos, get_todo},
    models::model_todo::{AllTodosTag, Todo, TodoId, TodoResponse},
};

// TODO. Create a [#hook] for this

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(non_snake_case)]
pub fn useTodoQuery() -> QueryScope<TodoId, TodoResponse> {
    create_query(
        get_todo,
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}

#[allow(non_snake_case)]
pub fn useAllTodosQuery() -> QueryScope<AllTodosTag, Vec<Todo>> {
    create_query(
        |_| async move { get_all_todos().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}
