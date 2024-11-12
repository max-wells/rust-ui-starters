use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn TextAlignLeft(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Text Align Left"</title>

            <path d="M15 12H3" />
            <path d="M17 18H3" />
            <path d="M21 6H3" />
        </SvgIcon>
    }
}
