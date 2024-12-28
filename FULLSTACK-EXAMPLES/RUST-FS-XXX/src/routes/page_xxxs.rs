use leptos::*;

use crate::components::{
    forms::{form_xxx_toast::FormXxxToast, form_xxx_validate::FormXxxValidate},
    leptos_query::all_xxxs::AllXxxs,
    toaster_custom::toaster::Toaster,
};

#[component]
pub fn PageXxxs() -> impl IntoView {
    view! {
        <Toaster>
            <div class="container flex flex-col gap-10 mx-auto mt-10">
                <p class="text-orange-500">"TODO 👉 Chopper 1 livre / virer local"</p>

                <div class="flex gap-10">
                    <FormXxxToast />
                    <FormXxxValidate />
                </div>

                <AllXxxs />
            </div>
        </Toaster>
    }
}
