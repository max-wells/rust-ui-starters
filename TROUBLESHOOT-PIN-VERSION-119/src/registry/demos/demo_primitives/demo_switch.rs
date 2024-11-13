use leptos::*;

#[cfg(feature = "hydrate")]
use crate::registry::primitives::p_switch::{PrimitiveSwitchRoot, PrimitiveSwitchThumb};

#[component]
pub fn DemoSwitch() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let switch_component = view! {
        <div class="flex gap-4 items-center">
            <label for="airplane-mode">"Airplane mode"</label>

            <PrimitiveSwitchRoot
                attr:class="focus-visible:ring-1 focus-visible:ring-ring focus-visible:ring-offset-1 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input focus-visible:outline-none focus-visible:ring-offset-background peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors"
                attr:id="airplane-mode"
            >
                // attr:style="-webkit-tap-highlight-color: rgba(0, 0, 0, 0)"
                <PrimitiveSwitchThumb attr:class="size-5 data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0 pointer-events-none block rounded-full bg-background shadow-lg ring-0 transition-transform" />
            </PrimitiveSwitchRoot>
        </div>
    };

    #[cfg(not(feature = "hydrate"))]
    let switch_component = view! {
        <p>
            "Switch functionality is not available on the server. This component requires client-side hydration."
        </p>
    };

    view! { <div>{switch_component}</div> }
}
