use leptos::*;

use crate::registry::ui::{
    button::Button,
    gradient::{Gradient, GradientVariant},
    meteor_effect::{MeteorCard, MeteorEffect, Meteors},
};

#[component]
pub fn DemoMeteorEffect() -> impl IntoView {
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
            <p class="relative z-20 mb-4 text-xl font-bold text-white">Meteor effect</p>
            <p class="relative z-20 mb-4 text-base font-normal text-slate-500">
                {"Lorem ipsum dolor sit amet consectetur adipisicing elit. Dolore esse eu aliqua elit aliqua dolor consectetur officia labore pariatur in eiusmod cupidatat nulla et. Reprehenderit ex aliqua nisi est mollit minim ut."}
            </p>
            <Button>Explore</Button>
        </div>
    }
}
