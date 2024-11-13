use leptos::*;

use crate::registry::ui::{
    gradient::{Gradient, GradientVariant},
    meteor_effect::{MeteorCard, MeteorEffect, Meteors},
};

#[component]
pub fn DemoGradient() -> impl IntoView {
    view! {
        <MeteorEffect>
            <Gradient variant=GradientVariant::BlueTeal />

            <MeteorCard>
                <DemoContent />
                <Meteors number=20 />
            </MeteorCard>
        </MeteorEffect>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DemoContent() -> impl IntoView {
    view! {
        <div>
            <p class="relative z-50 mb-4 text-xl font-bold text-white">Meteor effect</p>
            <p class="relative z-50 mb-4 text-base font-normal text-slate-500">
                {"Lorem ipsum dolor sit amet consectetur adipisicing elit. Dolore esse eu aliqua elit aliqua dolor consectetur officia labore pariatur in eiusmod cupidatat nulla et. Reprehenderit ex aliqua nisi est mollit minim ut."}
            </p>
            <button class="inline-flex z-10 justify-center items-center py-2 px-4 h-10 text-sm font-medium whitespace-nowrap rounded-md border transition-colors focus-visible:ring-1 focus-visible:ring-offset-1 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none ring-offset-background w-fit border-input bg-background hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring">
                Explore
            </button>
        </div>
    }
}
