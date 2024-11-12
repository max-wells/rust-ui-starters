use leptos_query::*;
use std::time::Duration;

use crate::{
    api::api_todos_complex::{get_all_todos_complex, get_todo_complex},
    models::model_todo_complex::{AllTodosTagComplex, TodoComplex, TodoIdComplex, TodoResponseComplex},
};

// TODO. Create a [#hook] for this

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ HELPERS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(non_snake_case)]
pub fn useTodoQueryComplex() -> QueryScope<TodoIdComplex, TodoResponseComplex> {
    create_query(
        get_todo_complex,
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}

#[allow(non_snake_case)]
pub fn useAllTodosQueryComplex() -> QueryScope<AllTodosTagComplex, Vec<TodoComplex>> {
    create_query(
        |_| async move { get_all_todos_complex().await.unwrap_or_default() },
        QueryOptions {
            stale_time: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    )
}
