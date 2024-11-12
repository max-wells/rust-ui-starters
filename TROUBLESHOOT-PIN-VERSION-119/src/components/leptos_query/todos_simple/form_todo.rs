use leptos::*;
use leptos_router::ActionForm;

use crate::{
    api::api_todos::AddTodo,
    models::model_todo::AllTodosTag,
    registry::ui::{button::Button, input::Input, label::Label},
    utils::hooks::queries::queries_todos::{useAllTodosQuery, useTodoQuery},
};

// TODO. Display loading state when adding todo.

#[component]
pub fn FormTodo() -> impl IntoView {
    let add_todo = create_server_action::<AddTodo>();
    let response = add_todo.value();

    let todo_query = useTodoQuery();
    let all_todos_query = useAllTodosQuery();

    create_effect(move |_| {
        // If action is successful.
        if let Some(Ok(todo)) = response.get() {
            all_todos_query.cancel_query(AllTodosTag);

            // Optimistic update for all todos.
            all_todos_query.update_query_data_mut(AllTodosTag, {
                let todo = todo.clone();
                |todos| {
                    todos.push(todo);
                }
            });

            // Optimistic update for individual TodoResponse.
            let id = todo.id;
            todo_query.set_query_data(id, Ok(Some(todo)));

            // Invalidate individual TodoResponse.
            todo_query.invalidate_query(id);

            // Invalidate AllTodos.
            all_todos_query.invalidate_query(AllTodosTag);
        }
    });

    // TODO Label component

    view! {
        <ActionForm action=add_todo>
            <div class="flex gap-2 items-center p-1 border max-w-[370px]">
                <Label r#for="todo-content">"Add a Todo "</Label>
                <Input id="todo-content" name="content" />
                <Button r#type="submit">"Add"</Button>
            </div>
        </ActionForm>
    }
}
