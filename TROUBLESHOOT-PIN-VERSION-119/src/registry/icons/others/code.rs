use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn CodeIcon(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Code"</title>

            <polyline points="16 18 22 12 16 6" />
            <polyline points="8 6 2 12 8 18" />
        </SvgIcon>
    }
}
