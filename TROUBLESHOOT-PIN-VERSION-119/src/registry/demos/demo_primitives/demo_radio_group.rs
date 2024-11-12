use leptos::*;

#[cfg(feature = "hydrate")]
use crate::registry::primitives::p_radio_group::{
    RadioGroupIndicator, RadioGroupItem, RadioGroupRoot,
};

// TODO. Remove hydrate

#[component]
pub fn DemoRadioGroup() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let radio_group_component = view! {
        <RadioGroupRoot
            attr:class="flex flex-col gap-2.5"
            default_value="default"
            attr:aria-label="View density"
        >
            <div class="flex gap-4 items-center">
                <RadioGroupItem
                    attr:class="bg-white rounded-full outline-none cursor-default size-6 hover:bg-neutral-300"
                    value="default"
                    attr:id="r1"
                >
                    <RadioGroupIndicator attr:class="flex items-center justify-center size-full relative after:content-[''] after:block after:w-[11px] after:h-[11px] after:rounded-[50%] after:bg-neutral-700" />
                </RadioGroupItem>
                <label for="r1">"Default"</label>
            </div>

            <div class="flex gap-4 items-center">
                <RadioGroupItem
                    attr:class="bg-white rounded-full outline-none cursor-default size-6 hover:bg-neutral-300"
                    value="comfortable"
                    attr:id="r2"
                >
                    <RadioGroupIndicator attr:class="flex items-center justify-center size-full relative after:content-[''] after:block after:w-[11px] after:h-[11px] after:rounded-[50%] after:bg-neutral-700" />
                </RadioGroupItem>
                <label for="r2">"Comfortable"</label>
            </div>

            <div class="flex gap-4 items-center">
                <RadioGroupItem
                    attr:class="bg-white rounded-full outline-none cursor-default size-6 hover:bg-neutral-300"
                    value="compact"
                    attr:id="r3"
                >
                    <RadioGroupIndicator attr:class="flex items-center justify-center size-full relative after:content-[''] after:block after:w-[11px] after:h-[11px] after:rounded-[50%] after:bg-neutral-700" />
                </RadioGroupItem>
                <label for="r3">"Compact"</label>
            </div>
        </RadioGroupRoot>
    };

    #[cfg(not(feature = "hydrate"))]
    let radio_group_component = view! {
        <p>
            "Radio group functionality is not available on the server. This component requires client-side hydration."
        </p>
    };

    view! { <div>{radio_group_component}</div> }
}
