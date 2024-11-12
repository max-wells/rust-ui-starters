use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn ChevronRight(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Chevron Right"</title>

            <path d="m9 18 6-6-6-6" />
        </SvgIcon>
    }
}
