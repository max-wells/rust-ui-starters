use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn ChevronUpDown(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Chevron Up Down"</title>

            <path d="m7 15 5 5 5-5" />
            <path d="m7 9 5-5 5 5" />
        </SvgIcon>
    }
}
