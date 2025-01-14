use leptos::html::Div;
use leptos::{
    component, create_node_ref, create_signal, view, IntoView, Show, SignalGet, SignalSet,
};

use crate::registry::ui::button::Button;

#[cfg(feature = "hydrate")]
use leptos_use::on_click_outside;

#[component]
pub fn DemoUseClickOutside() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let modal_ref = create_node_ref::<Div>();

    #[cfg(feature = "hydrate")]
    {
        let _ = on_click_outside(modal_ref, move |_| set_show_modal.set(false));
    }

    view! {
        <Button on:click=move |_| set_show_modal.set(true)>"Open Modal"</Button>

        <Show when=move || show_modal.get() fallback=|| ()>
            <div node_ref=modal_ref class="modal">
                <div class="inner">
                    <button
                        class="button small"
                        title="Close"
                        on:click=move |_| set_show_modal.set(false)
                    >
                        "𝖷"
                    </button>
                    <p class="heading">"Demo Modal"</p>
                    <p>"Click outside this modal to close it."</p>
                </div>
            </div>
        </Show>

        <style>
            "
            .modal {
            position: fixed;
            left: 50%;
            top: 50%;
            transform: translate(-50%, -50%);
            width: 420px;
            max-width: 100%;
            z-index: 10;
            }
            .inner {
            background-color: var(--bg);
            padding: 0.4em 2em;
            border-radius: 5px;
            border: 1px solid var(--theme-popup-border);
            box-shadow: 2px 2px 10px rgba(10, 10, 10, 0.1);
            }
            .dropdown-inner {
            background-color: var(--bg);
            padding: 0.5em;
            position: absolute;
            left: 0;
            z-index: 10;
            border-radius: 5px;
            border: 1px solid var(--theme-popup-border);
            box-shadow: 2px 2px 5px rgba(10, 10, 10, 0.1);
            }
            .heading {
            font-weight: bold;
            font-size: 1.4rem;
            margin-bottom: 2rem;
            }
            .modal > .inner > .button {
            position: absolute;
            top: 0;
            right: 0;
            margin: 0;
            font-weight: bold;
            }
            "
        </style>
    }
}
