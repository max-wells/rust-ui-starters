use leptos::*;

use crate::{
    components::toaster_custom::toaster::Toaster,
    features::xxxs::components::xxxs_display_all::XxxsDisplayAll,
    features::xxxs::components::xxxs_form_toast::XxxsFormToast,
    features::xxxs::components::xxxs_form_validate::XxxsFormValidate,
};

#[component]
pub fn PageXxxs() -> impl IntoView {
    view! {
        <Toaster>
            <div class="container flex flex-col gap-10 mx-auto mt-10">
                <div class="flex gap-10">
                    <XxxsFormToast />
                    <XxxsFormValidate />
                </div>

                <XxxsDisplayAll />
            </div>
        </Toaster>
    }
}
