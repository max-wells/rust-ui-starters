use leptos::*;

use crate::components::leptos_query::users_json::all_users_json::AllUsersJson;

#[component]
pub fn PageLeptosQueryUsersJson() -> impl IntoView {
    view! {
        <div class="container flex flex-col gap-10 mx-auto">
            // <div class="flex gap-10">
            // <TodoWithQueryComplex />
            // </div>
            // <FormTodoComplex />
            <AllUsersJson />
        </div>
    }
}
