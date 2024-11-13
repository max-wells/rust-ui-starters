use leptos::*;
use tailwind_fuse::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn MarqueeWrapper(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex overflow-hidden relative flex-col justify-center items-center p-20 w-full h-full rounded-lg border md:shadow-xl min-h-[300px] bg-background",
            class()
        )
    });

    view! {
        <div {..attributes} class=class>
            {children()}
            <div class="absolute inset-y-0 left-0 w-1/3 bg-gradient-to-r from-white pointer-events-none dark:from-background"></div>
            <div class="absolute inset-y-0 right-0 w-1/3 bg-gradient-to-l from-white pointer-events-none dark:from-background"></div>
        </div>
    }
}

#[component]
pub fn Marquee(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex overflow-hidden flex-row p-2 group [--gap:1rem] [gap:var(--gap)] [--duration:20s]",
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
pub fn MarqueeRow(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex flex-row justify-around shrink-0 [gap:var(--gap)] animate-marquee group-hover:[animation-play-state:paused]",
            class()
        )
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}
