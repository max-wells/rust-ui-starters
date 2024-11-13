use crate::registry::ui::avatar_simple::{AvatarImageSimple, AvatarRootSimple};
use leptos::*;

#[component]
pub fn DemoAvatarSimple() -> impl IntoView {
    view! {
        <div class="flex gap-5">
            <AvatarRootSimple class="rounded-md">
                <AvatarImageSimple
                    src="https://images.unsplash.com/photo-1511485977113-f34c92461ad9?ixlib=rb-1.2.1&w=128&h=128&dpr=2&q=80"
                    alt="Pedro Duarte"
                />
            </AvatarRootSimple>

            <AvatarRootSimple>
                <AvatarImageSimple
                    src="https://images.unsplash.com/photo-1511485977113-f34c92461ad9?ixlib=rb-1.2.1&w=128&h=128&dpr=2&q=80"
                    alt="Pedro Duarte"
                />
            </AvatarRootSimple>
        </div>
    }
}
