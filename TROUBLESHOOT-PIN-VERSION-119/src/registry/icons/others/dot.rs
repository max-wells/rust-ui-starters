use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn Dot(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Dot"</title>

            <circle cx="12.1" cy="12.1" r="1" />
        </SvgIcon>
    }
}
