use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn Circle(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Circle"</title>

            <circle cx="12" cy="12" r="10" />
        </SvgIcon>
    }
}