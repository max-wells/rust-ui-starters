use leptos::*;
use leptos_meta::Stylesheet;

use tailwind_fuse::*;

#[component]
pub fn DemoRadarMini() -> impl IntoView {
    view! {
        <Stylesheet id="radar-mini" href="/components/radar-mini.css" />

        <RadarMini>
            <RadarSpinner />
        </RadarMini>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn RadarMini(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex overflow-hidden relative justify-center items-center bg-transparent rounded-full border border-solid loader w-[150px] h-[150px] shadow-[25px_25px_75px_rgba(0,0,0,0.55)] border-[#333]",
            " before:content-[''] before:absolute before:inset-[20px] before:bg-transparent before:border before:border-dashed before:border-[#444] before:rounded-full before:shadow-[inset_-5px_-5px_25px_rgba(0,0,0,0.25),_inset_5px_5px_35px_rgba(0,0,0,0.25)]",
            "after:content-[''] after:absolute after:w-[50px] after:h-[50px] after:rounded-full after:border after:border-dashed after:border-[#444] after:shadow-[inset_-5px_-5px_25px_rgba(0,0,0,0.25),_inset_5px_5px_35px_rgba(0,0,0,0.25)]",
            class()
        )
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn RadarSpinner(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "absolute top-1/2 left-1/2 w-1/2 h-full bg-transparent border-t border-white border-dashed origin-top-left",
            "animate-radarMini",
            class()
        )
    });

    view! { <span {..attributes} class=class /> }
}
