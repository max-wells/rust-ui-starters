use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn TextAlignRight(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Text Align Right"</title>

            <path d="M21 12H9" />
            <path d="M21 18H7" />
            <path d="M21 6H3" />
        </SvgIcon>
    }
}
