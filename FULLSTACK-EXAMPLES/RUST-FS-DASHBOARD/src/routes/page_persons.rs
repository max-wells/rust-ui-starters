use leptos::*;

use crate::components::{
    forms::{form_persons_toast::FormPersonsToast, form_persons_validate::FormPersonsValidate},
    leptos_query::all_persons::AllPersons,
    toaster_custom::toaster::Toaster,
};

#[component]
pub fn PagePersons() -> impl IntoView {
    view! {
        <Toaster>
            <div class="container flex flex-col gap-10 mx-auto mt-10">
                <div class="flex gap-10">
                    <FormPersonsToast />
                    <FormPersonsValidate />
                </div>

                <AllPersons />
            </div>
        </Toaster>
    }
}
