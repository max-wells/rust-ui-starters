use leptos::*;
use leptos_router::A as Link;

use crate::registry::icons::others::home::HomeIcon;

#[component]
pub fn LogoHomeLink() -> impl IntoView {
    view! {
        <Link href="/">
            <HomeIcon />
        </Link>
    }
}
