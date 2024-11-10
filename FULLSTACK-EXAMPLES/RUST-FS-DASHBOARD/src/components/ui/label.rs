use leptos::*;
use tailwind_fuse::*;

use crate::components::ui::_shared::STYLES;

#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] r#for: String,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        STYLES.DISABLED_NOT_ALLOWED_PEER,
        "text-sm font-medium leading-none",
        class
    );

    view! {
        <label class=class r#for=r#for>
            {children()}
        </label>
    }
}
