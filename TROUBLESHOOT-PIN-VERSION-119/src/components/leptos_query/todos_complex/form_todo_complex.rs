use leptos::*;
use leptos_router::ActionForm;
use wasm_bindgen::JsCast; // Import the JsCast trait

use crate::{
    api::api_todos_complex::AddTodoComplex,
    models::model_todo_complex::AllTodosTagComplex,
    registry::ui::{button::Button, input::Input, label::Label},
    utils::hooks::queries::queries_todos_complex::{useAllTodosQueryComplex, useTodoQueryComplex},
};

// TODO. Display loading state when adding todo.

#[component]
pub fn FormTodoComplex() -> impl IntoView {
    let add_todo_complex = create_server_action::<AddTodoComplex>();
    let response = add_todo_complex.value();

    let todo_query = useTodoQueryComplex();
    let all_todos_query = useAllTodosQueryComplex();

    create_effect(move |_| {
        // If action is successful.
        if let Some(Ok(todo)) = response.get() {
            all_todos_query.cancel_query(AllTodosTagComplex);

            // Optimistic update for all todos.
            all_todos_query.update_query_data_mut(AllTodosTagComplex, {
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
            all_todos_query.invalidate_query(AllTodosTagComplex);
        }
    });

    // Function to handle form submission and alert the content
    let handle_submit = move |event: web_sys::SubmitEvent| {
        event.prevent_default();
        let form_data = web_sys::FormData::new_with_form(
            &event
                .target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlFormElement>(),
        )
        .unwrap();
        let content = form_data.get("content").as_string().unwrap_or_default();
        let description = form_data.get("description").as_string().unwrap_or_default();
        web_sys::window()
            .unwrap()
            .alert_with_message(&format!(
                "Sending: {{ content: {}, description: {} }}",
                content, description
            ))
            .unwrap();

        // Create an instance of AddTodoComplex with the form data
        let new_todo = AddTodoComplex {
            content: content.clone(),
            description: description.clone(),
        };

        add_todo_complex.dispatch(new_todo);
    };

    view! {
        <ActionForm action=add_todo_complex on:submit=handle_submit>
            <div class="flex flex-col gap-2 p-1 border max-w-[370px]">
                <div class="flex gap-2 items-center">
                    <Label r#for="todo-content">"Todo"</Label>
                    <Input id="todo-content" name="content" />
                </div>
                <div class="flex gap-2 items-center">
                    <Label r#for="todo-description">"Description"</Label>
                    <Input id="todo-description" name="description" />
                </div>
                <Button r#type="submit">"Add Todo"</Button>
            </div>
        </ActionForm>
    }
}
