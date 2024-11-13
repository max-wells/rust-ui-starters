use leptos::*;

use crate::registry::ui::button::Button;

#[component]
pub fn DemoPopoverModal() -> impl IntoView {
    view! {
        <Button popovertarget="mypopover">Open Popover</Button>

        <dialog
            id="mypopover"
            // * 💁 💡 Manual mode means can't close the popover by clicking outside of it.
            // * ---> It needs the popovertargetaction="hide" to be clicked.
            popover="manual"
            class="p-6 max-w-xs rounded-lg border-none"
            style="backdrop-filter: brightness(0.5);"
        >
            <button
                popovertarget="mypopover"
                popovertargetaction="hide"
                class="absolute top-4 right-4 bg-none border-none cursor-pointer outline-none filter-grayscale"
            >
                "❌"
            </button>
            <h3 class="mt-0">This is a headline</h3>
            <p class="text-gray-600">
                Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor.
            </p>

            <Button popovertarget="mypopover" popovertargetaction="hide">
                Continue
            </Button>
        </dialog>
    }
}
