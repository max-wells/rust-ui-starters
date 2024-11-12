use leptos::*;
use tailwind_fuse::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn Radar(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex overflow-hidden relative flex-col justify-center items-center space-y-4 w-full h-96",
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
pub fn RadarLine(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] style: MaybeSignal<String>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "absolute inset-0 top-1/2 left-1/2 rounded-full border transform -translate-x-1/2 -translate-y-1/2 size-10 border-neutral-200",
            class()
        )
    });

    view! { <div {..attributes} class=class style=style /> }
}

#[component]
pub fn RadarRow(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex justify-center items-center space-x-10 w-full md:justify-between md:space-x-0 mx-auto w-full",
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
pub fn RadarLinesContainer(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex relative -bottom-12 justify-center items-center rounded-full size-20",
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
pub fn RadarBottomLine(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "absolute bottom-0 w-full h-px bg-gradient-to-r from-transparent to-transparent z-[41] via-slate-700",
            class()
        )
    });

    view! { <div {..attributes} class=class /> }
}

#[component]
pub fn RadarSpinner(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "animate-radarSpin",
            "flex overflow-hidden absolute top-1/2 right-1/2 z-40 justify-center items-end bg-transparent h-[5px] w-[400px]",
            class()
        )
    });

    view! {
        <div {..attributes} class=class style="transform-origin:right center">
            <div class="relative z-40 w-full bg-gradient-to-r from-transparent to-transparent h-[1px] via-sky-600" />
        </div>
    }
}
