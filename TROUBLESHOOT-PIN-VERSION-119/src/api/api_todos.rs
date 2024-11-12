use leptos::*;

use crate::models::model_todo::{Todo, TodoId};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{sync::RwLock};
        static GLOBAL_TODOS: RwLock<Vec<Todo>> = RwLock::new(vec![]);
    }
}


/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[server(GetTodo, "/api")]
pub async fn get_todo(id: TodoId) -> Result<Option<Todo>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let todos = GLOBAL_TODOS.read().unwrap();
    Ok(todos.iter().find(|todo| todo.id == id).cloned())
}

#[server(GetTodos, "/api")]
pub async fn get_all_todos() -> Result<Vec<Todo>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let todos = GLOBAL_TODOS.read().unwrap();
    Ok(todos.clone())
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ MUTATIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[server(AddTodo, "/api")]
pub async fn add_todo(content: String) -> Result<Todo, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let mut todos = GLOBAL_TODOS.write().unwrap();

    let new_id = todos
        .last()
        .map(|t| t.id.0 + 1)
        .map(TodoId)
        .unwrap_or(TodoId(0));

    let new_todo = Todo {
        id: new_id,
        content,
    };

    todos.push(new_todo.clone());

    Ok(new_todo)
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: TodoId) -> Result<(), ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let mut todos = GLOBAL_TODOS.write().unwrap();
    todos.retain(|t| t.id != id);
    Ok(())
}
