use futures::future::join_all;
use leptos::*;
use prelude::ServerFnError;

use crate::features::{auth::auth_services::get_user, todos::todos_models::Todo};
// use crate::utils_ssr::ssr::{pool, SqlTodo}; // TODO  be able to have the import here

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use crate::utils_ssr::ssr::{pool, SqlTodo}; // TODO  remove this import and put it above

    let pool = pool()?;

    Ok(join_all(
        sqlx::query_as::<_, SqlTodo>("SELECT * FROM todos")
            .fetch_all(&pool)
            .await?
            .iter()
            .map(|todo: &SqlTodo| todo.clone().into_todo(&pool)),
    )
    .await)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    use crate::utils_ssr::ssr::pool; // TODO  remove this import and put it above

    let user = get_user().await?;
    let pool = pool()?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    Ok(
        sqlx::query("INSERT INTO todos (title, user_id, completed) VALUES (?, ?, false)")
            .bind(title)
            .bind(id)
            .execute(&pool)
            .await
            .map(|_| ())?,
    )
}

#[server]
pub async fn delete_todo(id: u16) -> Result<(), ServerFnError> {
    use crate::utils_ssr::ssr::pool; // TODO  remove this import and put it above

    let pool = pool()?;

    Ok(sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())?)
}
