use leptos::*;

use crate::components::icons::_icon_shared::SvgIcon;

#[component]
pub fn ChevronDown(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Chevron Down"</title>

            <path d="m6 9 6 6 6-6" />
        </SvgIcon>
    }
}
