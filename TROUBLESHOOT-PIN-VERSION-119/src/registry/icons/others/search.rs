use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn SearchIcon(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Search"</title>

            <circle cx="11" cy="11" r="8" />
            <path d="m21 21-4.3-4.3" />
        </SvgIcon>
    }
}
