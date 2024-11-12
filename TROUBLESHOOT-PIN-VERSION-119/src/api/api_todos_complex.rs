use leptos::*;

use crate::models::model_todo_complex::{TodoComplex, TodoIdComplex};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{sync::RwLock};
        static GLOBAL_TODOS: RwLock<Vec<TodoComplex>> = RwLock::new(vec![]);
    }
}


/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[server(GetTodoComplex, "/api")]
pub async fn get_todo_complex(id: TodoIdComplex) -> Result<Option<TodoComplex>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let todos = GLOBAL_TODOS.read().unwrap();
    Ok(todos.iter().find(|todo| todo.id == id).cloned())
}

#[server(GetTodosComplex, "/api")]
pub async fn get_all_todos_complex() -> Result<Vec<TodoComplex>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let todos = GLOBAL_TODOS.read().unwrap();
    Ok(todos.clone())
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ MUTATIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[server(AddTodoComplex, "/api")]
pub async fn add_todo_complex(content: String, description: String) -> Result<TodoComplex, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let mut todos = GLOBAL_TODOS.write().unwrap();

    let new_id = todos
        .last()
        .map(|t| t.id.0 + 1)
        .map(TodoIdComplex)
        .unwrap_or(TodoIdComplex(0));

    let new_todo = TodoComplex {
        id: new_id,
        content,
        description,
    };

    todos.push(new_todo.clone());

    Ok(new_todo)
}

#[server(DeleteTodoComplex, "/api")]
pub async fn delete_todo_complex(id: TodoIdComplex  ) -> Result<(), ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let mut todos = GLOBAL_TODOS.write().unwrap();
    todos.retain(|t| t.id != id);
    Ok(())
}
