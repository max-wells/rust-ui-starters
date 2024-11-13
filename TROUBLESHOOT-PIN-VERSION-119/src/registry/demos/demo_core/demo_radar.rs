use leptos::*;

use crate::registry::{
    icons::others::file_question::FileQuestion,
    ui::radar::{Radar, RadarBottomLine, RadarLine, RadarLinesContainer, RadarRow, RadarSpinner},
};

// TODO UI. Improve the radar demo.

#[component]
pub fn DemoRadar() -> impl IntoView {
    view! {
        <Radar class="max-w-3xl">
            <DemoRadarIcons />
            <DemoRadarLines />

            <RadarBottomLine />
        </Radar>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
fn DemoRadarIcons() -> impl IntoView {
    view! {
        <div class="w-full h-full">
            <RadarRow class="max-w-3xl">
                <DemoIcon />
                <DemoIcon />
                <DemoIcon />
            </RadarRow>

            <RadarRow class="max-w-md">
                <DemoIcon />
                <DemoIcon />
            </RadarRow>

            <RadarRow class="max-w-3xl">
                <DemoIcon />
                <DemoIcon />
                <DemoIcon />
            </RadarRow>
        </div>
    }
}

#[component]
fn DemoRadarLines() -> impl IntoView {
    view! {
        <RadarLinesContainer>
            <RadarSpinner />

            <RadarLine style="height:5rem;width:5rem;border:1px solid rgba(71, 85, 105, 0.9);" />
            <RadarLine style="height:10rem;width:10rem;border:1px solid rgba(71, 85, 105, 0.8);" />
            <RadarLine style="height:15rem;width:15rem;border:1px solid rgba(71, 85, 105, 0.7);" />
            <RadarLine style="height:20rem;width:20rem;border:1px solid rgba(71, 85, 105, 0.6);" />
            <RadarLine style="height:25rem;width:25rem;border:1px solid rgba(71, 85, 105, 0.5);" />
            <RadarLine style="height:30rem;width:30rem;border:1px solid rgba(71, 85, 105, 0.3999999999999999);" />
            <RadarLine style="height:35rem;width:35rem;border:1px solid rgba(71, 85, 105, 0.29999999999999993);" />
            <RadarLine style="height:40rem;width:40rem;border:1px solid rgba(71, 85, 105, 0.19999999999999996);" />
        </RadarLinesContainer>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
fn DemoIcon() -> impl IntoView {
    view! {
        <div class="flex relative z-50 flex-col justify-center items-center space-y-2">
            <div class="flex flex-col justify-center items-center">
                <div class="flex justify-center items-center rounded-2xl border shadow-inner border-slate-700 bg-slate-800 size-12">
                    <FileQuestion class="text-slate-600 size-8" />
                </div>
                <div class="hidden py-1 px-2 rounded-md md:block">
                    <p class="text-xs font-bold text-center text-slate-400">Demo Icon</p>
                </div>
            </div>
        </div>
    }
}
