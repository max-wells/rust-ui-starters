use leptos::*;

use crate::{
    components::homepage_components::SeparatorWaveLine,
    constants::homepage_demo_code::HOMEPAGE_DEMO_CODE,
    registry::{
        demos::{
            demo_extensions::demo_cards_slider::DemoCardsSlider,
            demo_core::{
                demo_faq::DemoFaq, demo_marquee::DemoMarquee, demo_meteor_effect::DemoMeteorEffect,
                demo_orbiting_circles::DemoOrbitingCircles, demo_radar::DemoRadar,
            },
        },
        ui::{
            animate::{Animate, AnimateVariant},
            headings::{HeadingVariant, H1},
        },
    },
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-14">
            <main class="flex relative gap-14 justify-center mx-auto w-full min-h-screen homeMain">
                <div class="relative mt-14 w-full h-fit max-w-[640px] homeContent">
                    <Animate
                        variant=AnimateVariant::FadeUp
                        style="animation-delay: 250ms; animation-fill-mode: forwards;"
                    >
                        <H1 variant=HeadingVariant::Modern class="text-center">
                            "Rust UI, your Leptos component library"
                        </H1>
                    </Animate>

                    <SeparatorWaveLine />

                    <CodeSection />
                </div>
            </main>
            <BentoSection />

            <DemoFaq />
        </div>
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

                <code>{HOMEPAGE_DEMO_CODE}</code>
            </pre>
        </div>
    }
}

#[component]
pub fn BentoSection() -> impl IntoView {
    view! {
        <div class="p-2 border border-sky-500">
            <div class="flex gap-2">
                <div class="flex-grow-[2]">
                    <DemoRadar />
                </div>
                <div class="flex-grow">
                    <DemoMeteorEffect />
                </div>
            </div>

            <DemoMarquee />

            <DemoCardsSlider />

            <DemoOrbitingCircles />
        </div>
    }
}
