use leptos::*;

use crate::{
    components::homepage_components::SeparatorWaveLine,
    registry::ui::{
        animate::{Animate, AnimateVariant},
        headings::{HeadingVariant, H1},
    },
};

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <main class="flex relative justify-center mx-auto w-full min-h-screen homeMain">
            <div class="relative mt-14 w-full h-fit max-w-[640px] homeContent">
                <Animate
                    variant=AnimateVariant::FadeUp
                    style="animation-delay: 0.25s; animation-fill-mode: forwards;"
                >
                    <H1 variant=HeadingVariant::Modern>"Not Found"</H1>
                </Animate>

                <SeparatorWaveLine />

                <CodeSection />
            </div>
        </main>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn CodeSection() -> impl IntoView {
    view! {
        <div class="relative mt-[72px] codeBlock">
            <div class="absolute top-0 w-[300px] h-[1px] codeBlockLine2" aria-hidden />
            <div
                class="absolute h-[300px] top-[-16px] left-[-16px] rounded-t-[16px] codeBlockLine3"
                aria-hidden
            />
            <pre class="relative p-4 text-xs rounded-[12px] codeBlockRoot">
                <div class="absolute h-[1px] w-[97%] top-[-1px] codeBlockShine" />

                // <code>{HOMEPAGE_DEMO_CODE}</code>
                <div class="h-[300px]" />

            </pre>
        </div>
    }
}
