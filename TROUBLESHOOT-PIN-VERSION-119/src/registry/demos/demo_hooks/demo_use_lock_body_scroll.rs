use leptos::*;

use crate::registry::hooks::use_lock_body_scroll::use_lock_body_scroll;
use crate::registry::ui::button::Button;

#[component]
pub fn DemoUseLockBodyScroll() -> impl IntoView {
    let (locked, set_locked) = use_lock_body_scroll(false);

    let toggle_lock = move |_| {
        set_locked.set(!locked.get());
    };

    view! {
        <div class="h-[1200px]">
            <Button on:click=toggle_lock>
                {move || if locked.get() { "Unlock Scroll" } else { "Lock Scroll" }}
            </Button>
        </div>
    }
}
