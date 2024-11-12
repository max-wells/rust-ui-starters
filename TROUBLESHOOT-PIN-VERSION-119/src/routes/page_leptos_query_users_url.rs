use leptos::*;

use crate::components::leptos_query::users_url::{
    all_users_url::AllUsersUrl, all_users_url_from_client::AllUsersUrlFromClient,
};

#[component]
pub fn PageLeptosQueryUsersUrl() -> impl IntoView {
    view! {
        <div class="container flex flex-col gap-10 mx-auto">
            // <div class="flex gap-10">
            // <TodoWithQueryComplex />
            // </div>
            // <FormTodoComplex />

            <div class="flex gap-4">
                <AllUsersUrl />
                <AllUsersUrlFromClient />
            </div>
        </div>
    }
}
