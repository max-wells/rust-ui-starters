use leptos::*;

use crate::registry::ui::spotlight::{Spotlight, SpotlightContainer};

#[component]
pub fn DemoSpotlight() -> impl IntoView {
    view! {
        <SpotlightContainer class="rounded-md h-[40rem] bg-black/80">
            <Spotlight class="left-0 -top-40 md:-top-20 md:left-60" />

            <DemoContent />
        </SpotlightContainer>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
fn DemoContent() -> impl IntoView {
    view! {
        <div class="relative z-10 p-4 pt-20 mx-auto w-full max-w-7xl text-center md:pt-0">
            <p class="mb-4 text-4xl font-bold bg-opacity-50 md:text-7xl">Spotlight effect</p>
            <p class="mx-auto max-w-lg font-normal text-neutral-300">
                {"Aliquip excepteur elit aliquip labore labore elit magna velit laboris exercitation deserunt. Laboris dolor duis sunt incididunt nostrud est eu sit ex non. Elit veniam occaecat do est."}
            </p>
        </div>
    }
}
