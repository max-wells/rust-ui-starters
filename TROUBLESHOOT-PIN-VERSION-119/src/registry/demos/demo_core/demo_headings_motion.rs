use leptos::*;

use crate::registry::ui::{
    animate::{Animate, AnimateVariant},
    button::Button,
    headings::H1,
};

#[component]
pub fn DemoHeadingsMotion() -> impl IntoView {
    // Create a state variable to trigger rerender
    let (rerender_key, set_rerender_key) = create_signal(0);

    view! {
        <div class="flex flex-col gap-2">
            // Use the state variable to force remount
            <Animate
                variant=AnimateVariant::FadeUp
                style="animation-delay: 0.25s; animation-fill-mode: forwards;"
            >
                // Use key to force remount
                // Set key to the current value of rerender_key
                <H1 key=rerender_key
                    .get()>{move || format!("This is a Motion Heading {}", rerender_key.get())}</H1>
            </Animate>
            // Display the number of renders
            <p>{move || format!("Render count: {}", rerender_key.get())}</p>
            // Button to update the state variable and force remount
            <Button on:click=move |_| {
                set_rerender_key.set(rerender_key.get() + 1);
                log::info!("Rerender key updated to: {}", rerender_key.get());
            }>"Rerender Heading"</Button>
        </div>
    }
}
