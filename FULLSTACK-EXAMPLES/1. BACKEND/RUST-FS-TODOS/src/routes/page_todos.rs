use leptos::*;

use crate::components::{all_todos::AllTodos, all_todos_from_client::AllTodosFromClient};

#[component]
pub fn PageTodos() -> impl IntoView {
    view! {
        <div class="container flex flex-col gap-10 mx-auto mt-10">
            // <div class="flex gap-10">
            // <TodoWithQueryComplex />
            // </div>
            // <FormTodoComplex />

            <div class="flex gap-4">
                <AllTodos />
                <AllTodosFromClient />
            </div>
        </div>
    }
}
