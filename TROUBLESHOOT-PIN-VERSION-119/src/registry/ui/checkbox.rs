use leptos::*;
use tailwind_fuse::*;

use crate::registry::ui::_shared::STYLES;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] class: String,
    #[prop(optional)] checked: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_change: Option<Box<dyn Fn(bool)>>,
) -> impl IntoView {
    let (is_checked, set_is_checked) = create_signal(checked);

    let checkbox_class = tw_merge!(
        STYLES.RING_OFFSET_BG,
        STYLES.DISABLED_NOT_ALLOWED,
        STYLES.FOCUS_VISIBLE_RING,
        STYLES.BORDER_PRIMARY,
        STYLES.CHECKBOX_DATA_STATE_PRIMARY,
        "peer size-4 shrink-0 rounded-sm",
        class
    );

    let indicator_class = tw_merge!("text-current", STYLES.SIZE_FULL);

    view! {
        <div
            class=checkbox_class
            data-state=move || if is_checked() { "checked" } else { "unchecked" }
            on:click=move |_| {
                if !disabled {
                    set_is_checked.update(|v| *v = !*v);
                    if let Some(f) = on_change.as_ref() {
                        f(is_checked());
                    }
                }
            }
        >
            <input
                id=id
                type="checkbox"
                class="sr-only"
                prop:checked=is_checked
                disabled=disabled
            />
            <div class=indicator_class>
                {move || {
                    if is_checked() {
                        Some(
                            view! {
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    class="w-4 h-4"
                                >
                                    <polyline points="20 6 9 17 4 12"></polyline>
                                </svg>
                            },
                        )
                    } else {
                        None
                    }
                }}
            </div>
        </div>
    }
}
