use leptos::*;

use crate::components::leptos_query::server_books::{
    all_server_books::AllServerBooks, all_server_books_from_client::AllServerBooksFromClient,
    form_server_book_toast::FormServerBookToast, form_server_book_validate::FormServerBookValidate,
    form_server_books::FormServerBooks,
};

#[component]
pub fn PageLeptosQueryServerBooks() -> impl IntoView {
    view! {
        <div class="container flex flex-col gap-10 mx-auto">
            // <div class="flex gap-10">
            // <TodoWithQueryComplex />
            // </div>
            <div class="flex gap-10">
                <FormServerBooks />
                <FormServerBookToast />
                <FormServerBookValidate />
            </div>

            <div class="flex gap-4">
                <AllServerBooks />
                <AllServerBooksFromClient />
            </div>
        </div>
    }
}
