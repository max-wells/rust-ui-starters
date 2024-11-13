use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn TextAlignCenter(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Text Align Center"</title>

            <path d="M3 12h18" />
            <path d="M3 18h18" />
            <path d="M3 6h18" />
        </SvgIcon>
    }
}
