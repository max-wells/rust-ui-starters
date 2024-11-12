use leptos::*;

use crate::registry::{
    icons::others::rotate_ccw::RotateCcw,
    ui::{
        animate::{AnimateGroup, AnimateGroupItem, AnimateVariant},
        button::{Button, ButtonSize, ButtonVariant},
        headings::{HeadingVariant, H1},
    },
};

// TODO DEMO. The trigger does not seem to work... 🤔 Figure out why

#[component]
pub fn DemoAnimateGroup() -> impl IntoView {
    let (trigger, set_trigger) = create_signal(false);

    let handle_click = move || {
        set_trigger.update(|value| *value = !*value);
    };

    view! {
        <AnimateGroup class="flex relative flex-col gap-8 items-center w-full max-w-4xl h-80">
            <Button
                class="absolute top-0 right-0 z-10"
                on:click=move |_| handle_click()
                size=ButtonSize::Icon
            >
                <RotateCcw class="size-4" />
            </Button>

            <AnimateGroupItem
                variant=AnimateVariant::FadeUp
                delay_ms=250
                key=trigger.get().to_string()
            >
                <H1 variant=HeadingVariant::Modern class="mt-6 text-center text-transparent">
                    "The only UI website"
                </H1>
            </AnimateGroupItem>
            <AnimateGroupItem
                variant=AnimateVariant::FadeUp
                class="mx-auto max-w-lg"
                delay_ms=450
                key=trigger.get().to_string()
            >
                <p class="text-base font-normal text-center text-neutral-300">
                    "Unstyled highly composable components that you can copy/paste in your own
                    codebase. Built with Tailwind CSS and a bit of Framer Motion. Customize them as
                    you want."
                </p>
            </AnimateGroupItem>

            <AnimateGroupItem
                variant=AnimateVariant::FadeUp
                class="space-x-4"
                delay_ms=600
                key=trigger.get().to_string()
            >
                <Button>"Components"</Button>
                <Button variant=ButtonVariant::Outline>"Get started"</Button>
            </AnimateGroupItem>
        </AnimateGroup>
    }
}
