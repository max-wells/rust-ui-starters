use leptos::*;

use crate::components::leptos_query::todos_json::all_todos_json::AllTodosJson;

#[component]
pub fn PageLeptosQueryTodosJson() -> impl IntoView {
    view! {
        <div class="container flex flex-col gap-10 mx-auto">
            // <div class="flex gap-10">
            // <TodoWithQueryComplex />
            // </div>
            // <FormTodoComplex />
            <AllTodosJson />
        </div>
    }
}
