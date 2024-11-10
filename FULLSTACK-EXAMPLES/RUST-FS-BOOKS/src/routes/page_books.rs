use leptos::*;

use crate::components::{
    forms::{form_books_toast::FormBooksToast, form_books_validate::FormBooksValidate},
    leptos_query::all_books::AllBooks,
    toaster_custom::toaster::Toaster,
};

#[component]
pub fn PageBooks() -> impl IntoView {
    view! {
        <Toaster>
            <div class="container flex flex-col gap-10 mx-auto mt-10">
                <p class="text-orange-500">"TODO 👉 Chopper 1 livre / virer local"</p>

                <div class="flex gap-10">
                    <FormBooksToast />
                    <FormBooksValidate />
                </div>

                <AllBooks />
            </div>
        </Toaster>
    }
}
