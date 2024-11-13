use leptos::*;
use tailwind_fuse::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn OrbitingCircles(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex overflow-hidden relative justify-center items-center p-20 w-full rounded-lg border md:shadow-xl min-h-[300px] bg-background h-[500px]",
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
pub fn CircleItem(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] style: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex absolute justify-center items-center bg-transparent rounded-full border border-none transform-gpu animate-orbit [animation-delay:calc(var(--delay)*1000ms)] h-[30px] w-[30px] dark:bg-white/10",
            class()
        )
    });

    view! {
        <div {..attributes} class=class style=style>
            {children()}
        </div>
    }
}
