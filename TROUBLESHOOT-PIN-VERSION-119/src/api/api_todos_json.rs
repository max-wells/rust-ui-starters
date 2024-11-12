use leptos::*;

use crate::models::model_todo_json::{TodoIdJson, TodoJson};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{sync::RwLock};
        static GLOBAL_TODOS: RwLock<Vec<TodoJson>> = RwLock::new(vec![]);
    }
}





/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[server(GetTodoJson, "/api")]
pub async fn get_todo_json(id: TodoIdJson) -> Result<Option<TodoJson>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let todos = GLOBAL_TODOS.read().unwrap();
    Ok(todos.iter().find(|todo| todo.id == id).cloned())
}

#[server(GetTodosJson, "/api")]
pub async fn get_all_todos_json() -> Result<Vec<TodoJson>, ServerFnError> {
    // Fetch todos from the external URL
    let response = reqwest::get("http://localhost:3000/todos.json").await?;
    let todos: Vec<TodoJson> = response.json().await?;
    
    Ok(todos)
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ MUTATIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[server(AddTodoJson, "/api")]
pub async fn add_todo_json(content: String, description: String) -> Result<TodoJson, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let mut todos = GLOBAL_TODOS.write().unwrap();

    let new_id = todos
        .last()
        .map(|t| t.id.0 + 1)
        .map(TodoIdJson)
        .unwrap_or(TodoIdJson(0));

    let new_todo = TodoJson {
        id: new_id,
        content,
        description,
    };

    todos.push(new_todo.clone());

    Ok(new_todo)
}

#[server(DeleteTodoJson, "/api")]
pub async fn delete_todo_json(id: TodoIdJson) -> Result<(), ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let mut todos = GLOBAL_TODOS.write().unwrap();
    todos.retain(|t| t.id != id);
    Ok(())
}
