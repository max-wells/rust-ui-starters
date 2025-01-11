use leptos::*;

use crate::components::{
    forms::{form_blogs_toast::FormBlogsToast, form_blogs_validate::FormBlogsValidate},
    leptos_query::{all_blogs::AllBlogs, all_tags::AllTags},
    toaster_custom::toaster::Toaster,
};

#[component]
pub fn PageBlogs() -> impl IntoView {
    view! {
        <Toaster>
            <div class="container flex flex-col gap-10 mx-auto mt-10">
                <p class="text-orange-500">"TODO 👉 Chopper 1 livre / virer local"</p>

                <div class="flex gap-10">
                    <FormBlogsToast />
                // <FormBlogsValidate />
                </div>

                <AllTags />
                <AllBlogs />
            </div>
        </Toaster>
    }
}
