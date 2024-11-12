use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn Terminal(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Terminal"</title>

            <polyline points="4 17 10 11 4 5"></polyline>
            <line x1="12" x2="20" y1="19" y2="19"></line>
        </SvgIcon>
    }
}
