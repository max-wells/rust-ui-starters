use leptos::*;

use crate::{
    components::toaster_custom::toaster::Toaster,
    features::xxxs::components::xxxs_display_all::XxxsDisplayAll,
};

#[component]
pub fn PageXxxsServer() -> impl IntoView {
    view! {
        <Toaster>
            <div class="container flex flex-col gap-10 mx-auto mt-10">
                <XxxsDisplayAll />
            </div>
        </Toaster>
    }
}
